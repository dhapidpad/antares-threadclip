pub const SCHEMA_V1: &str = r#"
CREATE TABLE IF NOT EXISTS meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS account (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    token_expires_at INTEGER,
    last_import_at INTEGER
);

CREATE TABLE IF NOT EXISTS post (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL REFERENCES account(id),
    text TEXT NOT NULL DEFAULT '',
    author TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    permalink TEXT,
    media_type TEXT,
    reply_count INTEGER,
    like_count INTEGER,
    raw_json TEXT
);

CREATE TABLE IF NOT EXISTS reply (
    id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    replied_to_id TEXT,
    depth INTEGER NOT NULL,
    text TEXT NOT NULL DEFAULT '',
    author TEXT NOT NULL,
    is_author_reply INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    reply_count INTEGER,
    like_count INTEGER,
    raw_json TEXT
);

CREATE INDEX IF NOT EXISTS idx_reply_post ON reply(post_id, replied_to_id);

CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    color TEXT
);

CREATE TABLE IF NOT EXISTS content_tag (
    content_id TEXT NOT NULL,
    content_kind TEXT NOT NULL CHECK (content_kind IN ('post','reply')),
    tag_id INTEGER NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
    PRIMARY KEY (content_id, tag_id)
);

CREATE VIRTUAL TABLE IF NOT EXISTS content_fts USING fts5(
    text,
    content_id UNINDEXED,
    content_kind UNINDEXED,
    tokenize='unicode61 remove_diacritics 2'
);

CREATE TABLE IF NOT EXISTS saved_query (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    query_json TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS asset (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('image','video','audio','font')),
    path TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    license TEXT NOT NULL,
    attribution TEXT,
    source_url TEXT,
    duration_ms INTEGER,
    width INTEGER,
    height INTEGER
);

CREATE TABLE IF NOT EXISTS speaker_config (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    provider TEXT NOT NULL,
    voice_id TEXT NOT NULL,
    rate REAL NOT NULL DEFAULT 1.0,
    pitch REAL NOT NULL DEFAULT 0.0
);

CREATE TABLE IF NOT EXISTS project (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    dir TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    schema_version INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS render_job (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES project(id),
    preset TEXT NOT NULL,
    status TEXT NOT NULL,
    progress REAL NOT NULL DEFAULT 0,
    output_path TEXT,
    plan_hash TEXT,
    app_version TEXT,
    ffmpeg_version TEXT,
    error TEXT,
    started_at INTEGER,
    finished_at INTEGER
);
"#;
