use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingConfig {
    // Real score parameters
    pub weight_reply: f64,
    pub weight_like: f64,
    pub gravity: f64,

    // Heuristic score parameters
    pub weight_len: f64,
    pub weight_keywords: f64,
    pub weight_depth: f64,
    pub weight_author_reply: f64,
    pub keywords: Vec<String>,
}

impl Default for RankingConfig {
    fn default() -> Self {
        Self {
            weight_reply: 1.0,
            weight_like: 0.5,
            gravity: 1.5,
            weight_len: 0.4,
            weight_keywords: 0.3,
            weight_depth: 0.1,
            weight_author_reply: 0.2,
            keywords: vec![
                "ai".to_string(),
                "hay".to_string(),
                "review".to_string(),
                "hướng dẫn".to_string(),
            ],
        }
    }
}
