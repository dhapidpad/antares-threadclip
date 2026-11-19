use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tc_core::{Result, TcError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub id: String,
    pub name: String,
    pub language_code: String,
    pub gender: String, // "FEMALE" | "MALE"
    pub provider: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice_id: String,
    pub rate: f32,  // default 1.0
    pub pitch: f32, // default 0.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordTimepoint {
    pub word: String,
    pub start_ms: u64,
    pub end_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TtsResult {
    pub audio_bytes: Vec<u8>,
    pub duration_ms: u64,
    pub timepoints: Vec<WordTimepoint>,
}

pub trait TtsProvider: Send + Sync {
    fn list_voices(&self) -> Result<Vec<VoiceInfo>>;
    fn synthesize(&self, req: &TtsRequest) -> Result<TtsResult>;
}

pub struct MockTtsProvider;

impl MockTtsProvider {
    pub fn new() -> Self {
        Self
    }

    /// Generates a valid mono 16-bit 44.1kHz PCM WAV file
    fn generate_wav(duration_ms: u64, freq_hz: f32) -> Vec<u8> {
        let sample_rate = 44100u32;
        let num_samples = (sample_rate as u64 * duration_ms / 1000) as usize;
        let mut pcm_samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * freq_hz * 2.0 * std::f32::consts::PI).sin();
            let sample_i16 = (sample * 0.25 * 32767.0) as i16;
            pcm_samples.push(sample_i16);
        }

        let mut wav = Vec::new();
        // RIFF header
        wav.extend_from_slice(b"RIFF");
        let data_size = (pcm_samples.len() * 2) as u32;
        let file_size = 36 + data_size;
        wav.extend_from_slice(&file_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // fmt subchunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
        wav.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (1 for PCM)
        wav.extend_from_slice(&1u16.to_le_bytes()); // NumChannels (1 = mono)
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        let byte_rate = sample_rate * 2; // SampleRate * NumChannels * BitsPerSample/8
        wav.extend_from_slice(&byte_rate.to_le_bytes());
        wav.extend_from_slice(&2u16.to_le_bytes()); // BlockAlign (NumChannels * BitsPerSample/8)
        wav.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

        // data subchunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());
        for s in pcm_samples {
            wav.extend_from_slice(&s.to_le_bytes());
        }

        wav
    }
}

impl TtsProvider for MockTtsProvider {
    fn list_voices(&self) -> Result<Vec<VoiceInfo>> {
        Ok(vec![
            VoiceInfo {
                id: "vi-VN-Neural2-A".to_string(),
                name: "Giọng nữ miền Bắc (Neural2-A)".to_string(),
                language_code: "vi-VN".to_string(),
                gender: "FEMALE".to_string(),
                provider: "google".to_string(),
            },
            VoiceInfo {
                id: "vi-VN-Neural2-D".to_string(),
                name: "Giọng nam miền Nam (Neural2-D)".to_string(),
                language_code: "vi-VN".to_string(),
                gender: "MALE".to_string(),
                provider: "google".to_string(),
            },
            VoiceInfo {
                id: "mock-sine".to_string(),
                name: "Mock Voice (Offline Test)".to_string(),
                language_code: "vi-VN".to_string(),
                gender: "NEUTRAL".to_string(),
                provider: "mock".to_string(),
            },
        ])
    }

    fn synthesize(&self, req: &TtsRequest) -> Result<TtsResult> {
        let words: Vec<&str> = req.text.split_whitespace().collect();
        let word_count = words.len().max(1);
        let ms_per_word = (250.0 / req.rate.max(0.5)) as u64;
        let total_duration = (word_count as u64 * ms_per_word).max(500);

        let mut timepoints = Vec::new();
        let mut curr_ms = 0;
        for w in &words {
            let next_ms = curr_ms + ms_per_word;
            timepoints.push(WordTimepoint {
                word: w.to_string(),
                start_ms: curr_ms,
                end_ms: next_ms,
            });
            curr_ms = next_ms;
        }

        let audio = Self::generate_wav(total_duration, 440.0);
        Ok(TtsResult {
            audio_bytes: audio,
            duration_ms: total_duration,
            timepoints,
        })
    }
}

pub struct TtsCacheManager {
    cache_dir: PathBuf,
}

impl TtsCacheManager {
    pub fn new(cache_dir: impl AsRef<Path>) -> Self {
        let p = cache_dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&p);
        Self { cache_dir: p }
    }

    pub fn compute_hash(req: &TtsRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(req.text.as_bytes());
        hasher.update(req.voice_id.as_bytes());
        hasher.update(req.rate.to_le_bytes());
        hasher.update(req.pitch.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn get_or_synthesize(
        &self,
        provider: &dyn TtsProvider,
        req: &TtsRequest,
    ) -> Result<PathBuf> {
        let hash = Self::compute_hash(req);
        let file_path = self.cache_dir.join(format!("{}.wav", hash));

        if file_path.exists() {
            return Ok(file_path);
        }

        let res = provider.synthesize(req)?;
        fs::write(&file_path, &res.audio_bytes).map_err(TcError::Io)?;
        Ok(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_tts_synthesis_and_wav_header() {
        let provider = MockTtsProvider::new();
        let req = TtsRequest {
            text: "Xin chào bạn".to_string(),
            voice_id: "vi-VN-Neural2-A".to_string(),
            rate: 1.0,
            pitch: 0.0,
        };

        let res = provider.synthesize(&req).unwrap();
        assert!(res.duration_ms >= 500);
        assert_eq!(&res.audio_bytes[0..4], b"RIFF");
        assert_eq!(&res.audio_bytes[8..12], b"WAVE");
        assert_eq!(res.timepoints.len(), 3);
    }
}
