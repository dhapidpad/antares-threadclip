use crate::tree::{RawPostItem, TreeBuilder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tc_core::{Account, AccountId, Post, PostId, Result, TcError};
use tc_db::Database;

#[derive(Debug, serde::Deserialize)]
pub struct ImportFileFormat {
    pub account: Option<AccountHeader>,
    pub posts: Vec<RawPostItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AccountHeader {
    pub id: String,
    pub username: String,
}

pub struct Importer;

impl Importer {
    pub fn import_json_file(db: &Database, path: impl AsRef<Path>) -> Result<(usize, usize)> {
        let file = File::open(path).map_err(TcError::Io)?;
        let reader = BufReader::new(file);
        let data: ImportFileFormat =
            serde_json::from_reader(reader).map_err(TcError::Serialization)?;

        Self::import_data(db, data)
    }

    pub fn import_json_str(db: &Database, json_str: &str) -> Result<(usize, usize)> {
        let data: ImportFileFormat =
            serde_json::from_str(json_str).map_err(TcError::Serialization)?;

        Self::import_data(db, data)
    }

    pub fn import_data(db: &Database, data: ImportFileFormat) -> Result<(usize, usize)> {
        let account_id = match data.account {
            Some(acc) => {
                let account = Account {
                    id: AccountId::new(&acc.id),
                    username: acc.username,
                    token_expires_at: None,
                    last_import_at: Some(chrono::Utc::now().timestamp()),
                };
                db.upsert_account(&account)?;
                account.id
            }
            None => {
                let default_acc = Account {
                    id: AccountId::new("offline_user"),
                    username: "OfflineCreator".to_string(),
                    token_expires_at: None,
                    last_import_at: Some(chrono::Utc::now().timestamp()),
                };
                db.upsert_account(&default_acc)?;
                default_acc.id
            }
        };

        let mut posts_count = 0;
        let mut replies_count = 0;

        for raw_post in data.posts {
            let post = Post {
                id: PostId::new(&raw_post.id),
                account_id: account_id.clone(),
                text: raw_post.text,
                author: raw_post.author,
                created_at: raw_post.created_at,
                permalink: raw_post.permalink,
                media_type: None,
                reply_count: raw_post.reply_count,
                like_count: raw_post.like_count,
                raw_json: None,
            };

            let (_tree, replies) = TreeBuilder::build(post.clone(), raw_post.replies);

            // Upsert post into DB
            db.upsert_post(&post)?;
            posts_count += 1;

            // Upsert all replies into DB
            for r in replies {
                db.upsert_reply(&r)?;
                replies_count += 1;
            }
        }

        Ok((posts_count, replies_count))
    }
}
