use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tc_core::{Post, PostId, Reply, ReplyId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawReplyItem {
    pub id: String,
    pub replied_to_id: Option<String>,
    pub text: String,
    pub author: String,
    pub created_at: i64,
    pub reply_count: Option<i64>,
    pub like_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPostItem {
    pub id: String,
    pub text: String,
    pub author: String,
    pub created_at: i64,
    pub permalink: Option<String>,
    pub reply_count: Option<i64>,
    pub like_count: Option<i64>,
    pub replies: Vec<RawReplyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadTreeNode {
    pub reply: Reply,
    pub children: Vec<ThreadTreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadTree {
    pub post: Post,
    pub root_replies: Vec<ThreadTreeNode>,
    pub total_replies: usize,
    pub max_depth: u32,
}

pub struct TreeBuilder;

impl TreeBuilder {
    pub fn build(post: Post, raw_replies: Vec<RawReplyItem>) -> (ThreadTree, Vec<Reply>) {
        let author = post.author.clone();
        let post_id = post.id.clone();

        // Pass 1: Build map of parent_id -> children raw replies
        let mut by_parent: HashMap<Option<String>, Vec<RawReplyItem>> = HashMap::new();
        for r in raw_replies {
            by_parent
                .entry(r.replied_to_id.clone())
                .or_default()
                .push(r);
        }

        let mut flattened_replies: Vec<Reply> = Vec::new();
        let mut max_depth = 0u32;

        fn traverse(
            parent_id: Option<&str>,
            current_depth: u32,
            post_id: &PostId,
            post_author: &str,
            by_parent: &HashMap<Option<String>, Vec<RawReplyItem>>,
            flattened: &mut Vec<Reply>,
            max_depth: &mut u32,
        ) -> Vec<ThreadTreeNode> {
            let key = parent_id.map(|s| s.to_string());
            let items = match by_parent.get(&key) {
                Some(it) => it,
                None => return Vec::new(),
            };

            let mut nodes = Vec::new();
            for item in items {
                let is_author = item.author == post_author;
                let reply = Reply {
                    id: ReplyId::new(&item.id),
                    post_id: post_id.clone(),
                    replied_to_id: item.replied_to_id.clone(),
                    depth: current_depth,
                    text: item.text.clone(),
                    author: item.author.clone(),
                    is_author_reply: is_author,
                    created_at: item.created_at,
                    reply_count: item.reply_count,
                    like_count: item.like_count,
                    raw_json: None,
                };

                flattened.push(reply.clone());
                if current_depth > *max_depth {
                    *max_depth = current_depth;
                }

                let children = traverse(
                    Some(&item.id),
                    current_depth + 1,
                    post_id,
                    post_author,
                    by_parent,
                    flattened,
                    max_depth,
                );

                nodes.push(ThreadTreeNode { reply, children });
            }
            nodes
        }

        // Direct replies to post have replied_to_id == None OR replied_to_id == Some(post.id)
        let mut root_replies = traverse(
            None,
            1,
            &post_id,
            &author,
            &by_parent,
            &mut flattened_replies,
            &mut max_depth,
        );

        let post_id_str = post.id.as_str();
        let direct_replies_to_post = traverse(
            Some(post_id_str),
            1,
            &post_id,
            &author,
            &by_parent,
            &mut flattened_replies,
            &mut max_depth,
        );
        root_replies.extend(direct_replies_to_post);

        let total = flattened_replies.len();
        let tree = ThreadTree {
            post,
            root_replies,
            total_replies: total,
            max_depth,
        };

        (tree, flattened_replies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tc_core::{AccountId, PostId};

    #[test]
    fn test_tree_construction_and_depth() {
        let post = Post {
            id: PostId::new("p1"),
            account_id: AccountId::new("u1"),
            text: "Bài viết gốc".to_string(),
            author: "authorA".to_string(),
            created_at: 1000,
            permalink: None,
            media_type: None,
            reply_count: None,
            like_count: None,
            raw_json: None,
        };

        let raw_replies = vec![
            RawReplyItem {
                id: "r1".to_string(),
                replied_to_id: None,
                text: "Reply cấp 1".to_string(),
                author: "userB".to_string(),
                created_at: 1010,
                reply_count: None,
                like_count: None,
            },
            RawReplyItem {
                id: "r2".to_string(),
                replied_to_id: Some("r1".to_string()),
                text: "Reply cấp 2 từ chính tác giả".to_string(),
                author: "authorA".to_string(),
                created_at: 1020,
                reply_count: None,
                like_count: None,
            },
            RawReplyItem {
                id: "r3".to_string(),
                replied_to_id: Some("r2".to_string()),
                text: "Reply cấp 3".to_string(),
                author: "userC".to_string(),
                created_at: 1030,
                reply_count: None,
                like_count: None,
            },
        ];

        let (tree, flat) = TreeBuilder::build(post, raw_replies);
        assert_eq!(tree.total_replies, 3);
        assert_eq!(tree.max_depth, 3);
        assert_eq!(tree.root_replies.len(), 1);

        // Check depth and author reply flags
        let r1 = flat.iter().find(|r| r.id.as_str() == "r1").unwrap();
        assert_eq!(r1.depth, 1);
        assert!(!r1.is_author_reply);

        let r2 = flat.iter().find(|r| r.id.as_str() == "r2").unwrap();
        assert_eq!(r2.depth, 2);
        assert!(r2.is_author_reply);

        let r3 = flat.iter().find(|r| r.id.as_str() == "r3").unwrap();
        assert_eq!(r3.depth, 3);
        assert!(!r3.is_author_reply);
    }
}
