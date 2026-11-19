use crate::plan::{RenderPlan, RenderPreset};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use tc_core::Result;
use tc_timeline::{Clip, ContentSnapshot, Frame, ProjectModel, Track, TrackKind};

pub struct GoldenTestSuite;

impl GoldenTestSuite {
    pub fn create_sample_text_only_project() -> ProjectModel {
        let mut proj = ProjectModel::default();
        let clip = Clip {
            id: "clip-text-1".to_string(),
            asset_id: None,
            content_ref: Some(ContentSnapshot {
                kind: "post".to_string(),
                id: "p1".to_string(),
                text: "Khám phá video short cùng Antares ThreadClip Studio".to_string(),
                author: "antares_creator".to_string(),
            }),
            tts: None,
            start: Frame(0),
            duration: Frame(60), // 2 seconds at 30fps
            in_point: Frame(0),
            loop_media: false,
            style_id: Some("card-default".to_string()),
            transition_in: None,
            transition_out: None,
        };

        if let Some(track) = proj.tracks.iter_mut().find(|t| t.kind == TrackKind::Text) {
            track.clips.push(clip);
        }
        proj
    }

    pub fn compute_file_sha256(path: impl AsRef<Path>) -> Result<String> {
        let data = fs::read(path).map_err(tc_core::TcError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        Ok(format!("{:x}", hasher.finalize()))
    }
}
