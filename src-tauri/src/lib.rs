mod audio;
mod paste;
mod transcriber;
mod tray;

use audio::AudioRecorder;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use transcriber::Transcriber;

struct AppState {
    recorder: Mutex<AudioRecorder>,
    transcriber: Mutex<Option<Transcriber>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState {
            recorder: Mutex::new(AudioRecorder::new()),
            transcriber: Mutex::new(None),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            tray::setup_tray(app.handle())?;

            let model_path = transcriber::resolve_bundled_model(app.handle())
                .expect("Bundled whisper model not found");
            let state = app.state::<AppState>();
            match Transcriber::new(&model_path) {
                Ok(t) => {
                    *state.transcriber.lock().unwrap() = Some(t);
                    log::info!("Whisper model loaded successfully");
                }
                Err(e) => {
                    log::error!("Failed to load whisper model: {}", e);
                }
            }

            let shortcut = Shortcut::new(
                Some(Modifiers::META | Modifiers::ALT),
                Code::Slash,
            );

            let handle = app.handle().clone();
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, _shortcut, event| {
                        let handle = handle.clone();
                        match event.state() {
                            ShortcutState::Pressed => {
                                handle_hotkey_press(&handle);
                            }
                            ShortcutState::Released => {
                                handle_hotkey_release(&handle);
                            }
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(shortcut)?;

            log::info!("Show Me The Talk started. Hotkey: Cmd+Option+/");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_hotkey_press(handle: &tauri::AppHandle) {
    let state = handle.state::<AppState>();
    let result = {
        let mut recorder = match state.recorder.lock() {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to lock recorder: {}", e);
                let _ = handle.emit("transcription-error", e.to_string());
                return;
            }
        };
        recorder.start()
    };
    match result {
        Ok(()) => {
            log::info!("Recording started");
            let _ = handle.emit("recording-started", ());
        }
        Err(e) => {
            log::error!("Failed to start recording: {}", e);
            let _ = handle.emit("transcription-error", e);
        }
    }
}

fn handle_hotkey_release(handle: &tauri::AppHandle) {
    let state = handle.state::<AppState>();
    let wav_path = {
        let mut recorder = match state.recorder.lock() {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to lock recorder: {}", e);
                return;
            }
        };
        match recorder.stop_and_save() {
            Ok(path) => {
                let _ = handle.emit("recording-stopped", ());
                path
            }
            Err(e) => {
                log::error!("Failed to save recording: {}", e);
                let _ = handle.emit("transcription-error", e);
                return;
            }
        }
    };

    let handle = handle.clone();
    std::thread::spawn(move || {
        run_transcription(handle, wav_path);
    });
}

fn run_transcription(handle: tauri::AppHandle, wav_path: std::path::PathBuf) {
    let samples = match transcriber::read_wav_samples(&wav_path) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to read WAV: {}", e);
            let _ = handle.emit("transcription-error", format!("Failed to read audio: {}", e));
            let _ = std::fs::remove_file(&wav_path);
            return;
        }
    };

    let state = handle.state::<AppState>();
    let transcriber_guard = match state.transcriber.lock() {
        Ok(g) => g,
        Err(e) => {
            log::error!("Failed to lock transcriber: {}", e);
            let _ = handle.emit("transcription-error", "Internal error".to_string());
            let _ = std::fs::remove_file(&wav_path);
            return;
        }
    };

    let transcriber_ref = match transcriber_guard.as_ref() {
        Some(t) => t,
        None => {
            let _ = handle.emit("transcription-error", "Model not loaded".to_string());
            let _ = std::fs::remove_file(&wav_path);
            return;
        }
    };

    match transcriber_ref.transcribe(&samples) {
        Ok(text) => {
            let _ = std::fs::remove_file(&wav_path);

            if text.is_empty() {
                let _ = handle.emit("transcription-error", "No speech detected".to_string());
                return;
            }

            log::info!("Transcription: {}", text);

            if let Err(e) = handle.clipboard().write_text(&text) {
                log::error!("Failed to copy to clipboard: {}", e);
                let _ = handle.emit(
                    "transcription-error",
                    format!("Clipboard error: {}", e),
                );
                return;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));

            if let Err(e) = paste::simulate_paste() {
                log::error!("Failed to simulate paste: {}", e);
                let _ = handle.emit(
                    "transcription-error",
                    format!("Paste error: {} (text copied to clipboard)", e),
                );
                return;
            }

            #[derive(Clone, serde::Serialize)]
            struct TranscriptionResult {
                text: String,
            }

            let _ = handle.emit("transcription-result", TranscriptionResult { text });
        }
        Err(e) => {
            let _ = std::fs::remove_file(&wav_path);
            log::error!("Transcription failed: {}", e);
            let _ = handle.emit("transcription-error", format!("Transcription failed: {}", e));
        }
    }
}
