use std::path::Path;
use tauri::{path::BaseDirectory, Manager};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct Transcriber {
    context: WhisperContext,
}

impl Transcriber {
    pub fn new(model_path: &Path) -> Result<Self, String> {
        let params = WhisperContextParameters::default();
        let context = WhisperContext::new_with_params(
            model_path.to_str().ok_or("Invalid model path")?,
            params,
        )
        .map_err(|e| format!("Failed to load whisper model: {}", e))?;

        log::info!("Whisper model loaded from: {}", model_path.display());
        Ok(Self { context })
    }

    pub fn transcribe(&self, audio_samples: &[f32]) -> Result<String, String> {
        let mut state = self
            .context
            .create_state()
            .map_err(|e| format!("Failed to create whisper state: {}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_suppress_nst(true);
        params.set_n_threads(4);
        params.set_no_context(true);

        state
            .full(params, audio_samples)
            .map_err(|e| format!("Transcription failed: {}", e))?;

        let num_segments = state.full_n_segments();
        let mut result = String::new();

        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(text) = segment.to_str_lossy() {
                    result.push_str(&text);
                }
            }
        }

        Ok(result.trim().to_string())
    }
}

pub fn resolve_bundled_model(handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    if let Ok(env_path) = std::env::var("WHISPER_MODEL_PATH") {
        let path = std::path::PathBuf::from(&env_path);
        if path.exists() {
            return Ok(path);
        }
    }
    handle
        .path()
        .resolve("models/ggml-base.en.bin", BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve bundled model path: {}", e))
}

pub fn read_wav_samples(wav_path: &Path) -> Result<Vec<f32>, String> {
    let reader = hound::WavReader::open(wav_path).map_err(|e| e.to_string())?;
    let spec = reader.spec();

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .into_samples::<f32>()
            .filter_map(|s| s.ok())
            .collect(),
        hound::SampleFormat::Int => {
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
            reader
                .into_samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / max_val)
                .collect()
        }
    };

    if spec.channels > 1 {
        let mono: Vec<f32> = samples
            .chunks(spec.channels as usize)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
            .collect();
        Ok(mono)
    } else {
        Ok(samples)
    }
}
