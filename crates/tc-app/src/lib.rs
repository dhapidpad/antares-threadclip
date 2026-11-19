pub mod commands;
pub mod logging;
pub mod state;

pub use commands::{AppController, AppInfo};
pub use logging::LogRedactor;
pub use state::AppState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_controller_flow() {
        let state = AppState::in_memory();
        let info = AppController::get_info();
        assert_eq!(info.version, "3.0.0");

        let sample_json = r#"{
            "account": {"id": "acc_1", "username": "creator_vn"},
            "posts": [
                {
                    "id": "p100",
                    "text": "Khám phá Antares ThreadClip",
                    "author": "creator_vn",
                    "created_at": 1700000000,
                    "permalink": null,
                    "reply_count": 5,
                    "like_count": 12,
                    "replies": []
                }
            ]
        }"#;

        let (p_count, r_count) = AppController::import_offline_json(&state, sample_json).unwrap();
        assert_eq!(p_count, 1);
        assert_eq!(r_count, 0);

        let posts = AppController::list_posts(&state, 10).unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].author, "creator_vn");

        let score = AppController::score_post(&state, "p100").unwrap();
        assert_eq!(score.kind, tc_ranking::ScoreKind::Real);

        // Project and timeline
        let proj = AppController::create_project(&state, "Shorts #1").unwrap();
        assert_eq!(proj.schema_version, 1);

        // Diagnostics redaction
        let diag = AppController::export_diagnostic_bundle(&state).unwrap();
        assert!(!diag.contains("secret_token"));
        assert!(!diag.contains("user@creator.com"));
        assert!(diag.contains("[REDACTED_TOKEN]"));
        assert!(diag.contains("[REDACTED_EMAIL]"));
    }
}
