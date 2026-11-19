pub mod commands;
pub mod history;
pub mod model;

pub use commands::{AddClipCommand, DeleteClipCommand, MoveClipCommand, TimelineCommand};
pub use history::TimelineHistory;
pub use model::{
    AudioRole, CanvasConfig, CaptionCue, CaptionsData, Clip, ContentSnapshot, DuckingConfig,
    ProjectModel, Track, TrackKind, Transition, TtsClipData,
};

#[cfg(test)]
mod tests {
    use super::*;
    use tc_core::Frame;

    #[test]
    fn test_undo_redo_add_clip() {
        let mut project = ProjectModel::default();
        let mut history = TimelineHistory::new();

        let clip = Clip {
            id: "c1".to_string(),
            asset_id: None,
            content_ref: Some(ContentSnapshot {
                kind: "post".to_string(),
                id: "p1".to_string(),
                text: "Xin chào thế giới".to_string(),
                author: "alice".to_string(),
            }),
            tts: None,
            start: Frame(0),
            duration: Frame(90),
            in_point: Frame(0),
            loop_media: false,
            style_id: None,
            transition_in: None,
            transition_out: None,
        };

        let cmd = Box::new(AddClipCommand {
            track_id: "track-text".to_string(),
            clip: clip.clone(),
        });

        history.execute(cmd, &mut project).unwrap();
        assert_eq!(project.tracks[1].clips.len(), 1);

        // Undo
        assert!(history.undo(&mut project).unwrap());
        assert_eq!(project.tracks[1].clips.len(), 0);

        // Redo
        assert!(history.redo(&mut project).unwrap());
        assert_eq!(project.tracks[1].clips.len(), 1);
    }
}
