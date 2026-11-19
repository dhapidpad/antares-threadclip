use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tc_core::{Frame, RationalFps};
use tc_timeline::ProjectModel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderPreset {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: RationalFps,
    pub crf: u32,
    pub preset_speed: String,
    pub audio_bitrate: String,
}

impl RenderPreset {
    pub fn shorts_1080x1920() -> Self {
        Self {
            name: "shorts-1080x1920".to_string(),
            width: 1080,
            height: 1920,
            fps: RationalFps::FPS_30,
            crf: 18,
            preset_speed: "medium".to_string(),
            audio_bitrate: "192k".to_string(),
        }
    }

    pub fn shorts_720x1280() -> Self {
        Self {
            name: "shorts-720x1280".to_string(),
            width: 720,
            height: 1280,
            fps: RationalFps::FPS_30,
            crf: 20,
            preset_speed: "fast".to_string(),
            audio_bitrate: "128k".to_string(),
        }
    }

    pub fn draft_540x960() -> Self {
        Self {
            name: "draft-540x960".to_string(),
            width: 540,
            height: 960,
            fps: RationalFps::FPS_30,
            crf: 26,
            preset_speed: "veryfast".to_string(),
            audio_bitrate: "96k".to_string(),
        }
    }

    pub fn by_name(name: &str) -> Self {
        match name {
            "shorts-720x1280" => Self::shorts_720x1280(),
            "draft-540x960" => Self::draft_540x960(),
            _ => Self::shorts_1080x1920(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderPlan {
    pub project: ProjectModel,
    pub preset: RenderPreset,
    pub total_frames: Frame,
    pub plan_hash: String,
    pub renderer_version: String,
}

impl RenderPlan {
    pub const RENDERER_VERSION: &'static str = "3.0.0";

    pub fn new(project: ProjectModel, preset: RenderPreset) -> Self {
        let total_frames = project.total_duration();
        let plan_hash = Self::compute_plan_hash(&project, &preset);

        Self {
            project,
            preset,
            total_frames,
            plan_hash,
            renderer_version: Self::RENDERER_VERSION.to_string(),
        }
    }

    pub fn compute_plan_hash(project: &ProjectModel, preset: &RenderPreset) -> String {
        let mut hasher = Sha256::new();
        let proj_json = serde_json::to_string(project).unwrap_or_default();
        let preset_json = serde_json::to_string(preset).unwrap_or_default();

        hasher.update(proj_json.as_bytes());
        hasher.update(preset_json.as_bytes());
        hasher.update(Self::RENDERER_VERSION.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
