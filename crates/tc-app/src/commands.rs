use crate::logging::LogRedactor;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tc_core::{Post, PostId, Result, TcError};
use tc_ranking::{RankingEngine, ScoreBreakdown};
use tc_threads::{Importer, ThreadTree, TreeBuilder};
use tc_timeline::{AddClipCommand, Clip, ProjectModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub os: String,
}

pub struct AppController;

impl AppController {
    pub fn get_info() -> AppInfo {
        AppInfo {
            name: "Antares ThreadClip Studio".to_string(),
            version: "3.0.0".to_string(),
            os: std::env::consts::OS.to_string(),
        }
    }

    pub fn import_offline_json(state: &AppState, json_str: &str) -> Result<(usize, usize)> {
        Importer::import_json_str(&state.db, json_str)
    }

    pub fn list_posts(state: &AppState, limit: usize) -> Result<Vec<Post>> {
        state.db.list_posts(limit)
    }

    pub fn get_post_tree(state: &AppState, post_id: &str) -> Result<ThreadTree> {
        let pid = PostId::new(post_id);
        let post = state
            .db
            .get_post(&pid)?
            .ok_or_else(|| TcError::NotFound(format!("Post {}", post_id)))?;

        let replies = state.db.get_replies_for_post(&pid)?;
        let raw_replies: Vec<tc_threads::RawReplyItem> = replies
            .into_iter()
            .map(|r| tc_threads::RawReplyItem {
                id: r.id.0,
                replied_to_id: r.replied_to_id,
                text: r.text,
                author: r.author,
                created_at: r.created_at,
                reply_count: r.reply_count,
                like_count: r.like_count,
            })
            .collect();

        let (tree, _) = TreeBuilder::build(post, raw_replies);
        Ok(tree)
    }

    pub fn score_post(state: &AppState, post_id: &str) -> Result<ScoreBreakdown> {
        let pid = PostId::new(post_id);
        let post = state
            .db
            .get_post(&pid)?
            .ok_or_else(|| TcError::NotFound(format!("Post {}", post_id)))?;

        let cfg = state.ranking_config.lock().unwrap().clone();
        let now = chrono::Utc::now().timestamp();

        let breakdown = RankingEngine::score_item(
            post.reply_count,
            post.like_count,
            post.created_at,
            now,
            &post.text,
            0,
            true,
            &cfg,
        );

        Ok(breakdown)
    }

    pub fn create_project(state: &AppState, _name: &str) -> Result<ProjectModel> {
        let proj = ProjectModel::default();
        let mut active = state.active_project.lock().unwrap();
        *active = Some(proj.clone());
        Ok(proj)
    }

    pub fn add_clip(state: &AppState, track_id: &str, clip: Clip) -> Result<ProjectModel> {
        let mut active = state.active_project.lock().unwrap();
        let proj = active
            .as_mut()
            .ok_or_else(|| TcError::InvalidTimeline("No active project".into()))?;

        let cmd = Box::new(AddClipCommand {
            track_id: track_id.to_string(),
            clip,
        });

        let mut hist = state.history.lock().unwrap();
        hist.execute(cmd, proj)?;

        Ok(proj.clone())
    }

    pub fn undo(state: &AppState) -> Result<Option<ProjectModel>> {
        let mut active = state.active_project.lock().unwrap();
        let proj = match active.as_mut() {
            Some(p) => p,
            None => return Ok(None),
        };

        let mut hist = state.history.lock().unwrap();
        if hist.undo(proj)? {
            Ok(Some(proj.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn redo(state: &AppState) -> Result<Option<ProjectModel>> {
        let mut active = state.active_project.lock().unwrap();
        let proj = match active.as_mut() {
            Some(p) => p,
            None => return Ok(None),
        };

        let mut hist = state.history.lock().unwrap();
        if hist.redo(proj)? {
            Ok(Some(proj.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn export_diagnostic_bundle(state: &AppState) -> Result<String> {
        let raw = format!(
            "App: Antares ThreadClip Studio v3.0.0\nOS: {}\nDB Path: {:?}\nUser Token: Bearer secret_token_xyz123\nContact: user@creator.com",
            std::env::consts::OS,
            state.db.get_meta("app_version")?
        );
        let safe = LogRedactor::redact(&raw);
        Ok(safe)
    }

    pub fn filter_posts(state: &AppState, query: &tc_filter::FilterQuery) -> Result<Vec<String>> {
        let compiled = tc_filter::QueryCompiler::compile_posts(query)?;
        let conn = state.db.raw_conn();
        let mut stmt = conn
            .prepare(&compiled.sql)
            .map_err(|e| TcError::Db(e.to_string()))?;

        let rows = stmt
            .query_map(rusqlite::params_from_iter(compiled.params.iter()), |row| {
                row.get::<_, String>(0)
            })
            .map_err(|e| TcError::Db(e.to_string()))?;

        let mut ids = Vec::new();
        for r in rows {
            ids.push(r.map_err(|e| TcError::Db(e.to_string()))?);
        }
        Ok(ids)
    }

    pub fn build_render_plan(
        state: &AppState,
        preset: tc_render::RenderPreset,
    ) -> Result<tc_render::RenderPlan> {
        let active = state.active_project.lock().unwrap();
        let proj = active
            .as_ref()
            .ok_or_else(|| TcError::InvalidTimeline("No active project".into()))?;

        Ok(tc_render::RenderPlan::new(proj.clone(), preset))
    }

    pub fn validate_license(license: &str, attribution: Option<&str>) -> Result<()> {
        tc_assets::AssetManager::validate_license(license, attribution)
    }
}
