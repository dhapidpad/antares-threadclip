pub mod schema;

use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use tc-core::{
    Account, AccountId, Asset, AssetKind, ContentKind, MediaType, Post, PostId, ProjectId,
    RenderJob, RenderJobStatus, Reply, ReplyId, Result, SpeakerConfig, Tag, TcError,
};

pub struct Database {
    conn: Connection,
    path: PathBuf,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        if let Some(parent) = path_buf.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Connection::open(&path_buf)
            .map_err(|e| TcError::Db(format!("Failed to open database: {e}")))?;

        // WAL mode & Foreign keys
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;",
        )
        .map_err(|e| TcError::Db(format!("Failed to configure SQLite pragmas: {e}")))?;

        let db = Self {
            conn,
            path: path_buf,
        };
        db.migrate()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| TcError::Db(format!("Failed to open in-memory db: {e}")))?;

        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| TcError::Db(format!("Failed to configure pragmas: {e}")))?;

        let db = Self {
            conn,
            path: PathBuf::from(":memory:"),
        };
        db.migrate()?;
        Ok(db)
    }

    pub fn migrate(&self) -> Result<()> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| TcError::Db(e.to_string()))?;

        tx.execute_batch(schema::SCHEMA_V1)
            .map_err(|e| TcError::Db(format!("Migration v1 failed: {e}")))?;

        tx.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('schema_version', '1')",
            [],
        )
        .map_err(|e| TcError::Db(e.to_string()))?;

        tx.execute(
            "INSERT OR IGNORE INTO meta (key, value) VALUES ('app_version', '3.0.0')",
            [],
        )
        .map_err(|e| TcError::Db(e.to_string()))?;

        tx.commit().map_err(|e| TcError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn backup(&self, dest: impl AsRef<Path>) -> Result<()> {
        if self.path.to_str() == Some(":memory:") {
            return Ok(());
        }
        std::fs::copy(&self.path, dest)
            .map_err(|e| TcError::Db(format!("Backup failed: {e}")))?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM meta WHERE key = ?1")
            .map_err(|e| TcError::Db(e.to_string()))?;
        let res = stmt
            .query_row(params![key], |row| row.get(0))
            .optional()
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(res)
    }

    // --- Account Operations ---
    pub fn upsert_account(&self, acc: &Account) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO account (id, username, token_expires_at, last_import_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET
                   username = excluded.username,
                   token_expires_at = excluded.token_expires_at,
                   last_import_at = excluded.last_import_at",
                params![
                    acc.id.as_str(),
                    acc.username,
                    acc.token_expires_at,
                    acc.last_import_at
                ],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn get_account(&self, id: &AccountId) -> Result<Option<Account>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, username, token_expires_at, last_import_at FROM account WHERE id = ?1")
            .map_err(|e| TcError::Db(e.to_string()))?;

        let res = stmt
            .query_row(params![id.as_str()], |row| {
                Ok(Account {
                    id: AccountId::new(row.get::<_, String>(0)?),
                    username: row.get(1)?,
                    token_expires_at: row.get(2)?,
                    last_import_at: row.get(3)?,
                })
            })
            .optional()
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(res)
    }

    // --- Post Operations ---
    pub fn upsert_post(&self, post: &Post) -> Result<()> {
        let media_str = post.media_type.as_ref().map(|m| format!("{:?}", m));
        self.conn
            .execute(
                "INSERT INTO post (id, account_id, text, author, created_at, permalink, media_type, reply_count, like_count, raw_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT(id) DO UPDATE SET
                   text = excluded.text,
                   author = excluded.author,
                   created_at = excluded.created_at,
                   permalink = excluded.permalink,
                   media_type = excluded.media_type,
                   reply_count = excluded.reply_count,
                   like_count = excluded.like_count,
                   raw_json = excluded.raw_json",
                params![
                    post.id.as_str(),
                    post.account_id.as_str(),
                    post.text,
                    post.author,
                    post.created_at,
                    post.permalink,
                    media_str,
                    post.reply_count,
                    post.like_count,
                    post.raw_json,
                ],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        // Update FTS
        self.conn
            .execute(
                "INSERT INTO content_fts (text, content_id, content_kind) VALUES (?1, ?2, 'post')",
                params![post.text, post.id.as_str()],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        Ok(())
    }

    pub fn get_post(&self, id: &PostId) -> Result<Option<Post>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, account_id, text, author, created_at, permalink, media_type, reply_count, like_count, raw_json
                 FROM post WHERE id = ?1",
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        let res = stmt
            .query_row(params![id.as_str()], |row| {
                let media_str: Option<String> = row.get(6)?;
                let media_type = media_str.and_then(|s| match s.as_str() {
                    "Image" => Some(MediaType::Image),
                    "Video" => Some(MediaType::Video),
                    "Carousel" => Some(MediaType::Carousel),
                    _ => Some(MediaType::TextPost),
                });
                Ok(Post {
                    id: PostId::new(row.get::<_, String>(0)?),
                    account_id: AccountId::new(row.get::<_, String>(1)?),
                    text: row.get(2)?,
                    author: row.get(3)?,
                    created_at: row.get(4)?,
                    permalink: row.get(5)?,
                    media_type,
                    reply_count: row.get(7)?,
                    like_count: row.get(8)?,
                    raw_json: row.get(9)?,
                })
            })
            .optional()
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(res)
    }

    pub fn list_posts(&self, limit: usize) -> Result<Vec<Post>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, account_id, text, author, created_at, permalink, media_type, reply_count, like_count, raw_json
                 FROM post ORDER BY created_at DESC LIMIT ?1",
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        let rows = stmt
            .query_map(params![limit as i64], |row| {
                let media_str: Option<String> = row.get(6)?;
                let media_type = media_str.and_then(|s| match s.as_str() {
                    "Image" => Some(MediaType::Image),
                    "Video" => Some(MediaType::Video),
                    "Carousel" => Some(MediaType::Carousel),
                    _ => Some(MediaType::TextPost),
                });
                Ok(Post {
                    id: PostId::new(row.get::<_, String>(0)?),
                    account_id: AccountId::new(row.get::<_, String>(1)?),
                    text: row.get(2)?,
                    author: row.get(3)?,
                    created_at: row.get(4)?,
                    permalink: row.get(5)?,
                    media_type,
                    reply_count: row.get(7)?,
                    like_count: row.get(8)?,
                    raw_json: row.get(9)?,
                })
            })
            .map_err(|e| TcError::Db(e.to_string()))?;

        let mut posts = Vec::new();
        for r in rows {
            posts.push(r.map_err(|e| TcError::Db(e.to_string()))?);
        }
        Ok(posts)
    }

    // --- Reply Operations ---
    pub fn upsert_reply(&self, reply: &Reply) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO reply (id, post_id, replied_to_id, depth, text, author, is_author_reply, created_at, reply_count, like_count, raw_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                 ON CONFLICT(id) DO UPDATE SET
                   depth = excluded.depth,
                   text = excluded.text,
                   author = excluded.author,
                   is_author_reply = excluded.is_author_reply,
                   created_at = excluded.created_at,
                   reply_count = excluded.reply_count,
                   like_count = excluded.like_count,
                   raw_json = excluded.raw_json",
                params![
                    reply.id.as_str(),
                    reply.post_id.as_str(),
                    reply.replied_to_id,
                    reply.depth,
                    reply.text,
                    reply.author,
                    if reply.is_author_reply { 1 } else { 0 },
                    reply.created_at,
                    reply.reply_count,
                    reply.like_count,
                    reply.raw_json,
                ],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        // Update FTS
        self.conn
            .execute(
                "INSERT INTO content_fts (text, content_id, content_kind) VALUES (?1, ?2, 'reply')",
                params![reply.text, reply.id.as_str()],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        Ok(())
    }

    pub fn get_replies_for_post(&self, post_id: &PostId) -> Result<Vec<Reply>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, post_id, replied_to_id, depth, text, author, is_author_reply, created_at, reply_count, like_count, raw_json
                 FROM reply WHERE post_id = ?1 ORDER BY depth ASC, created_at ASC",
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        let rows = stmt
            .query_map(params![post_id.as_str()], |row| {
                let is_auth: i64 = row.get(6)?;
                Ok(Reply {
                    id: ReplyId::new(row.get::<_, String>(0)?),
                    post_id: PostId::new(row.get::<_, String>(1)?),
                    replied_to_id: row.get(2)?,
                    depth: row.get(3)?,
                    text: row.get(4)?,
                    author: row.get(5)?,
                    is_author_reply: is_auth != 0,
                    created_at: row.get(7)?,
                    reply_count: row.get(8)?,
                    like_count: row.get(9)?,
                    raw_json: row.get(10)?,
                })
            })
            .map_err(|e| TcError::Db(e.to_string()))?;

        let mut replies = Vec::new();
        for r in rows {
            replies.push(r.map_err(|e| TcError::Db(e.to_string()))?);
        }
        Ok(replies)
    }

    // --- FTS Search ---
    pub fn search_fts(&self, query: &str, limit: usize) -> Result<Vec<(String, ContentKind, String)>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT content_id, content_kind, snippet(content_fts, 0, '<b>', '</b>', '...', 15)
                 FROM content_fts WHERE content_fts MATCH ?1 LIMIT ?2",
            )
            .map_err(|e| TcError::Db(e.to_string()))?;

        let rows = stmt
            .query_map(params![query, limit as i64], |row| {
                let cid: String = row.get(0)?;
                let ckind_str: String = row.get(1)?;
                let snippet: String = row.get(2)?;
                let kind = if ckind_str == "reply" {
                    ContentKind::Reply
                } else {
                    ContentKind::Post
                };
                Ok((cid, kind, snippet))
            })
            .map_err(|e| TcError::Db(e.to_string()))?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r.map_err(|e| TcError::Db(e.to_string()))?);
        }
        Ok(results)
    }

    // --- Tags ---
    pub fn create_tag(&self, name: &str, color: Option<&str>) -> Result<i64> {
        self.conn
            .execute(
                "INSERT INTO tag (name, color) VALUES (?1, ?2)",
                params![name, color],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, color FROM tag ORDER BY name ASC")
            .map_err(|e| TcError::Db(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })
            .map_err(|e| TcError::Db(e.to_string()))?;
        let mut tags = Vec::new();
        for r in rows {
            tags.push(r.map_err(|e| TcError::Db(e.to_string()))?);
        }
        Ok(tags)
    }

    pub fn tag_content(&self, content_id: &str, kind: ContentKind, tag_id: i64) -> Result<()> {
        let kind_str = match kind {
            ContentKind::Post => "post",
            ContentKind::Reply => "reply",
        };
        self.conn
            .execute(
                "INSERT OR IGNORE INTO content_tag (content_id, content_kind, tag_id) VALUES (?1, ?2, ?3)",
                params![content_id, kind_str, tag_id],
            )
            .map_err(|e| TcError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn raw_conn(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_init_and_migrations() {
        let db = Database::open_in_memory().unwrap();
        let schema_ver = db.get_meta("schema_version").unwrap();
        assert_eq!(schema_ver.as_deref(), Some("1"));
    }

    #[test]
    fn test_post_reply_and_fts() {
        let db = Database::open_in_memory().unwrap();
        let acc = Account {
            id: AccountId::new("user_1"),
            username: "alice".to_string(),
            token_expires_at: Some(1800000000),
            last_import_at: Some(1700000000),
        };
        db.upsert_account(&acc).unwrap();

        let post = Post {
            id: PostId::new("p1"),
            account_id: acc.id.clone(),
            text: "Tìm hiểu AI trong sáng tạo video ngắn với Antares ThreadClip".to_string(),
            author: "alice".to_string(),
            created_at: 1700000000,
            permalink: Some("https://threads.net/@alice/post/p1".to_string()),
            media_type: Some(MediaType::TextPost),
            reply_count: Some(5),
            like_count: Some(20),
            raw_json: None,
        };
        db.upsert_post(&post).unwrap();

        let reply = Reply {
            id: ReplyId::new("r1"),
            post_id: post.id.clone(),
            replied_to_id: None,
            depth: 1,
            text: "Ý tưởng quá hay, hỗ trợ tiếng Việt có dấu rất chuẩn!".to_string(),
            author: "bob".to_string(),
            is_author_reply: false,
            created_at: 1700000100,
            reply_count: Some(0),
            like_count: Some(3),
            raw_json: None,
        };
        db.upsert_reply(&reply).unwrap();

        let fetched_post = db.get_post(&post.id).unwrap().unwrap();
        assert_eq!(fetched_post.author, "alice");

        let replies = db.get_replies_for_post(&post.id).unwrap();
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].author, "bob");

        // FTS search (test diacritics removal unicode61)
        let search_results = db.search_fts("Antares", 10).unwrap();
        assert!(!search_results.is_empty());
        assert_eq!(search_results[0].0, "p1");
    }

    #[test]
    fn test_sql_injection_defense() {
        let db = Database::open_in_memory().unwrap();
        let acc = Account {
            id: AccountId::new("hacker"),
            username: "evil'; DROP TABLE post; --".to_string(),
            token_expires_at: None,
            last_import_at: None,
        };
        db.upsert_account(&acc).unwrap();

        let fetched = db.get_account(&acc.id).unwrap().unwrap();
        assert_eq!(fetched.username, "evil'; DROP TABLE post; --");

        // Ensure post table still exists
        let post_list = db.list_posts(10).unwrap();
        assert!(post_list.is_empty());
    }
}
