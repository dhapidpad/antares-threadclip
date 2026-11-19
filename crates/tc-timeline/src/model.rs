use serde::{Deserialize, Serialize};
use tc_core::{Frame, RationalFps};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Video,
    Text,
    Audio,
    Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioRole {
    Voice,
    Music,
    Sfx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuckingConfig {
    pub enabled: bool,
    pub reduce_db: f32, // e.g. 12.0
}

impl Default for DuckingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            reduce_db: 12.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSnapshot {
    pub kind: String, // "post" | "reply"
    pub id: String,
    pub text: String,
    pub author: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtsClipData {
    pub speaker_id: String,
    pub text: String,
    pub audio_asset_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub transition_type: String, // "fade" | "slide_up" | "pop" | "cut"
    pub frames: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub asset_id: Option<String>,
    pub content_ref: Option<ContentSnapshot>,
    pub tts: Option<TtsClipData>,
    pub start: Frame,
    pub duration: Frame,
    #[serde(rename = "in", default)]
    pub in_point: Frame,
    #[serde(default)]
    pub loop_media: bool,
    pub style_id: Option<String>,
    pub transition_in: Option<Transition>,
    pub transition_out: Option<Transition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    #[serde(default)]
    pub role: Option<AudioRole>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub ducking: Option<DuckingConfig>,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasConfig {
    pub width: u32,
    pub height: u32,
    pub fps: RationalFps,
    pub background: String,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
            fps: RationalFps::FPS_30,
            background: "#000000".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptionCue {
    pub start: Frame,
    pub duration: Frame,
    pub text: String,
    #[serde(default)]
    pub timing_estimated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CaptionsData {
    pub format: String, // "srt" | "vtt"
    pub cues: Vec<CaptionCue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectModel {
    pub schema_version: u32,
    pub canvas: CanvasConfig,
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub captions: CaptionsData,
    pub seed: u64,
}

impl Default for ProjectModel {
    fn default() -> Self {
        Self {
            schema_version: 1,
            canvas: CanvasConfig::default(),
            tracks: vec![
                Track {
                    id: "track-video".to_string(),
                    kind: TrackKind::Video,
                    role: None,
                    locked: false,
                    ducking: None,
                    clips: Vec::new(),
                },
                Track {
                    id: "track-text".to_string(),
                    kind: TrackKind::Text,
                    role: None,
                    locked: false,
                    ducking: None,
                    clips: Vec::new(),
                },
                Track {
                    id: "track-voice".to_string(),
                    kind: TrackKind::Audio,
                    role: Some(AudioRole::Voice),
                    locked: false,
                    ducking: None,
                    clips: Vec::new(),
                },
                Track {
                    id: "track-music".to_string(),
                    kind: TrackKind::Audio,
                    role: Some(AudioRole::Music),
                    locked: false,
                    ducking: Some(DuckingConfig::default()),
                    clips: Vec::new(),
                },
            ],
            captions: CaptionsData::default(),
            seed: 12345,
        }
    }
}

impl ProjectModel {
    pub fn total_duration(&self) -> Frame {
        let mut max_frame = Frame(0);
        for track in &self.tracks {
            for clip in &track.clips {
                let end = Frame(clip.start.0 + clip.duration.0);
                if end > max_frame {
                    max_frame = end;
                }
            }
        }
        max_frame
    }
}
