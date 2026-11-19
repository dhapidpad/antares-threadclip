use crate::id::{AccountId, PostId, ReplyId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Post,
    Reply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    TextPost,
    Image,
    Video,
    Carousel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
    pub token_expires_at: Option<i64>,
    pub last_import_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub account_id: AccountId,
    pub text: String,
    pub author: String,
    pub created_at: i64,
    pub permalink: Option<String>,
    pub media_type: Option<MediaType>,
    pub reply_count: Option<i64>,
    pub like_count: Option<i64>,
    pub raw_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reply {
    pub id: ReplyId,
    pub post_id: PostId,
    pub replied_to_id: Option<String>,
    pub depth: u32,
    pub text: String,
    pub author: String,
    pub is_author_reply: bool,
    pub created_at: i64,
    pub reply_count: Option<i64>,
    pub like_count: Option<i64>,
    pub raw_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Image,
    Video,
    Audio,
    Font,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub kind: AssetKind,
    pub path: String,
    pub sha256: String,
    pub license: String,
    pub attribution: Option<String>,
    pub source_url: Option<String>,
    pub duration_ms: Option<i64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerConfig {
    pub id: String,
    pub label: String,
    pub provider: String,
    pub voice_id: String,
    pub rate: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderJobStatus {
    Queued,
    Running,
    Done,
    Failed,
    Cancelled,
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderJob {
    pub id: String,
    pub project_id: String,
    pub preset: String,
    pub status: RenderJobStatus,
    pub progress: f32,
    pub output_path: Option<String>,
    pub plan_hash: Option<String>,
    pub app_version: Option<String>,
    pub ffmpeg_version: Option<String>,
    pub error: Option<String>,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
}
