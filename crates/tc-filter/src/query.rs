use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicalOp {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchOp {
    Contains,
    Exact,
    Between,
    In,
    Gte,
    Lte,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterNode {
    Group {
        op: LogicalOp,
        children: Vec<FilterNode>,
    },
    Field {
        field: String,
        #[serde(rename = "match")]
        match_op: MatchOp,
        value: serde_json::Value,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortConfig {
    pub by: String, // "score_real" | "score_heuristic" | "created_at" | "reply_count" | "like_count"
    pub dir: String, // "asc" | "desc"
    pub fallback: Option<String>,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            by: "created_at".to_string(),
            dir: "desc".to_string(),
            fallback: Some("id".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterQuery {
    pub version: u32,
    pub root: Option<FilterNode>,
    #[serde(default)]
    pub sort: SortConfig,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    200
}

impl Default for FilterQuery {
    fn default() -> Self {
        Self {
            version: 1,
            root: None,
            sort: SortConfig::default(),
            limit: 200,
        }
    }
}
