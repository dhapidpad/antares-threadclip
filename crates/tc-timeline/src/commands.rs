use crate::model::{Clip, ProjectModel};
use tc_core::{Frame, Result, TcError};

pub trait TimelineCommand: Send + Sync {
    fn execute(&self, project: &mut ProjectModel) -> Result<()>;
    fn undo(&self, project: &mut ProjectModel) -> Result<()>;
}

pub struct AddClipCommand {
    pub track_id: String,
    pub clip: Clip,
}

impl TimelineCommand for AddClipCommand {
    fn execute(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        // Validation: No overlap on same track
        let new_start = self.clip.start.0;
        let new_end = new_start + self.clip.duration.0;
        for c in &track.clips {
            let start = c.start.0;
            let end = start + c.duration.0;
            if !(new_end <= start || new_start >= end) {
                return Err(TcError::InvalidTimeline("Clip overlap on track".into()));
            }
        }

        track.clips.push(self.clip.clone());
        track.clips.sort_by_key(|c| c.start);
        Ok(())
    }

    fn undo(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        track.clips.retain(|c| c.id != self.clip.id);
        Ok(())
    }
}

pub struct MoveClipCommand {
    pub track_id: String,
    pub clip_id: String,
    pub old_start: Frame,
    pub new_start: Frame,
}

impl TimelineCommand for MoveClipCommand {
    fn execute(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        let clip = track
            .clips
            .iter_mut()
            .find(|c| c.id == self.clip_id)
            .ok_or_else(|| TcError::NotFound(format!("Clip {}", self.clip_id)))?;

        clip.start = self.new_start;
        track.clips.sort_by_key(|c| c.start);
        Ok(())
    }

    fn undo(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        let clip = track
            .clips
            .iter_mut()
            .find(|c| c.id == self.clip_id)
            .ok_or_else(|| TcError::NotFound(format!("Clip {}", self.clip_id)))?;

        clip.start = self.old_start;
        track.clips.sort_by_key(|c| c.start);
        Ok(())
    }
}

pub struct DeleteClipCommand {
    pub track_id: String,
    pub deleted_clip: Clip,
}

impl TimelineCommand for DeleteClipCommand {
    fn execute(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        track.clips.retain(|c| c.id != self.deleted_clip.id);
        Ok(())
    }

    fn undo(&self, project: &mut ProjectModel) -> Result<()> {
        let track = project
            .tracks
            .iter_mut()
            .find(|t| t.id == self.track_id)
            .ok_or_else(|| TcError::NotFound(format!("Track {}", self.track_id)))?;

        track.clips.push(self.deleted_clip.clone());
        track.clips.sort_by_key(|c| c.start);
        Ok(())
    }
}
