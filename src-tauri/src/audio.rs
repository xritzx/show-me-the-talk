use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const TARGET_SAMPLE_RATE: u32 = 16000;

pub struct AudioRecorder {
    samples: Arc<Mutex<Vec<f32>>>,
    stream: Option<cpal::Stream>,
    is_recording: Arc<Mutex<bool>>,
    device_sample_rate: u32,
    device_channels: u16,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            samples: Arc::new(Mutex::new(Vec::new())),
            stream: None,
            is_recording: Arc::new(Mutex::new(false)),
            device_sample_rate: 0,
            device_channels: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "No input device found".to_string())?;

        let default_config = device
            .default_input_config()
            .map_err(|e| format!("No default input config: {}", e))?;

        let config: cpal::StreamConfig = default_config.into();
        self.device_sample_rate = config.sample_rate;
        self.device_channels = config.channels;

        log::info!(
            "Mic: {}Hz, {} channels",
            self.device_sample_rate,
            self.device_channels
        );

        {
            let mut samples = self.samples.lock().map_err(|e| e.to_string())?;
            samples.clear();
        }

        let samples = Arc::clone(&self.samples);
        let is_recording = Arc::clone(&self.is_recording);

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(recording) = is_recording.lock() {
                        if *recording {
                            if let Ok(mut buf) = samples.lock() {
                                buf.extend_from_slice(data);
                            }
                        }
                    }
                },
                move |err| {
                    log::error!("Audio stream error: {}", err);
                },
                None,
            )
            .map_err(|e| format!("Failed to build input stream: {}", e))?;

        stream.play().map_err(|e| format!("Failed to start stream: {}", e))?;

        *self.is_recording.lock().map_err(|e| e.to_string())? = true;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop_and_save(&mut self) -> Result<PathBuf, String> {
        *self.is_recording.lock().map_err(|e| e.to_string())? = false;
        self.stream = None;

        let raw_samples = self.samples.lock().map_err(|e| e.to_string())?;
        if raw_samples.is_empty() {
            return Err("No audio recorded".to_string());
        }

        let mono = to_mono(&raw_samples, self.device_channels);
        let resampled = resample(&mono, self.device_sample_rate, TARGET_SAMPLE_RATE);

        let temp_dir = std::env::temp_dir().join("show_me_the_talk");
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

        let filename = format!("recording_{}.wav", chrono_timestamp());
        let wav_path = temp_dir.join(filename);

        let spec = WavSpec {
            channels: 1,
            sample_rate: TARGET_SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let file = std::fs::File::create(&wav_path).map_err(|e| e.to_string())?;
        let writer = BufWriter::new(file);
        let mut wav_writer = WavWriter::new(writer, spec).map_err(|e| e.to_string())?;

        for &sample in resampled.iter() {
            wav_writer.write_sample(sample).map_err(|e| e.to_string())?;
        }
        wav_writer.finalize().map_err(|e| e.to_string())?;

        log::info!(
            "Saved WAV: {} ({} samples, {}Hz -> {}Hz)",
            wav_path.display(),
            resampled.len(),
            self.device_sample_rate,
            TARGET_SAMPLE_RATE
        );
        Ok(wav_path)
    }
}

fn to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return samples.to_vec();
    }
    let ch = channels as usize;
    samples
        .chunks(ch)
        .map(|frame| frame.iter().sum::<f32>() / ch as f32)
        .collect()
}

/// Linear interpolation resampler (simple, good enough for speech).
fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate {
        return samples.to_vec();
    }
    let ratio = from_rate as f64 / to_rate as f64;
    let output_len = (samples.len() as f64 / ratio) as usize;
    let mut output = Vec::with_capacity(output_len);
    for i in 0..output_len {
        let src_idx = i as f64 * ratio;
        let idx = src_idx as usize;
        let frac = src_idx - idx as f64;
        let s0 = samples[idx];
        let s1 = if idx + 1 < samples.len() {
            samples[idx + 1]
        } else {
            s0
        };
        output.push(s0 + (s1 - s0) * frac as f32);
    }
    output
}

fn chrono_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
