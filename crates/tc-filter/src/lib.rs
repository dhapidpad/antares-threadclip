pub mod compiler;
pub mod query;

pub use compiler::{CompiledQuery, QueryCompiler};
pub use query::{FilterNode, FilterQuery, LogicalOp, MatchOp, SortConfig};

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params_from_iter;
    use tc_core::{Account, AccountId, MediaType, Post, PostId};
    use tc_db::Database;

    #[test]
    fn test_compile_and_execute_filter() {
        let db = Database::open_in_memory().unwrap();
        let acc = Account {
            id: AccountId::new("u1"),
            username: "tech_creator".to_string(),
            token_expires_at: None,
            last_import_at: None,
        };
        db.upsert_account(&acc).unwrap();

        let post1 = Post {
            id: PostId::new("p1"),
            account_id: acc.id.clone(),
            text: "Video short AI trong sáng tạo nội dung".to_string(),
            author: "tech_creator".to_string(),
            created_at: 1000,
            permalink: None,
            media_type: Some(MediaType::TextPost),
            reply_count: Some(10),
            like_count: Some(50),
            raw_json: None,
        };
        let post2 = Post {
            id: PostId::new("p2"),
            account_id: acc.id.clone(),
            text: "Thời tiết hôm nay đẹp quá".to_string(),
            author: "other_user".to_string(),
            created_at: 2000,
            permalink: None,
            media_type: Some(MediaType::TextPost),
            reply_count: Some(1),
            like_count: Some(2),
            raw_json: None,
        };
        db.upsert_post(&post1).unwrap();
        db.upsert_post(&post2).unwrap();

        let query = FilterQuery {
            version: 1,
            root: Some(FilterNode::Group {
                op: LogicalOp::And,
                children: vec![
                    FilterNode::Field {
                        field: "text".to_string(),
                        match_op: MatchOp::Contains,
                        value: serde_json::json!("AI trong"),
                    },
                    FilterNode::Field {
                        field: "reply_count".to_string(),
                        match_op: MatchOp::Gte,
                        value: serde_json::json!(5),
                    },
                ],
            }),
            sort: SortConfig {
                by: "created_at".to_string(),
                dir: "desc".to_string(),
                fallback: None,
            },
            limit: 50,
        };

        let compiled = QueryCompiler::compile_posts(&query).unwrap();
        let mut stmt = db.raw_conn().prepare(&compiled.sql).unwrap();

        let rows = stmt
            .query_map(params_from_iter(compiled.params.iter()), |row| {
                row.get::<_, String>(0)
            })
            .unwrap();

        let ids: Vec<String> = rows.map(|r| r.unwrap()).collect();
        assert_eq!(ids, vec!["p1"]);
    }
}
