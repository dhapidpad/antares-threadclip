use crate::config::RankingConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreKind {
    Real,
    Heuristic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactorContribution {
    pub name: String,
    pub raw_value: f64,
    pub weight: f64,
    pub contribution: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub total_score: f64,
    pub kind: ScoreKind,
    pub label: String,
    pub factors: Vec<FactorContribution>,
}

pub struct RankingEngine;

impl RankingEngine {
    pub fn compute_real_score(
        reply_count: Option<i64>,
        like_count: Option<i64>,
        created_at: i64,
        now: i64,
        config: &RankingConfig,
    ) -> Option<ScoreBreakdown> {
        if reply_count.is_none() && like_count.is_none() {
            return None;
        }

        let r_count = reply_count.unwrap_or(0) as f64;
        let l_count = like_count.unwrap_or(0) as f64;

        let age_hours = ((now - created_at) as f64 / 3600.0).max(0.0);
        let decay = (age_hours + 2.0).powf(config.gravity);

        let reply_contrib = (r_count * config.weight_reply) / decay;
        let like_contrib = (l_count * config.weight_like) / decay;
        let total = reply_contrib + like_contrib;

        Some(ScoreBreakdown {
            total_score: total,
            kind: ScoreKind::Real,
            label: "● Dữ liệu thật".to_string(),
            factors: vec![
                FactorContribution {
                    name: "Số phản hồi".to_string(),
                    raw_value: r_count,
                    weight: config.weight_reply,
                    contribution: reply_contrib,
                },
                FactorContribution {
                    name: "Số lượt thích".to_string(),
                    raw_value: l_count,
                    weight: config.weight_like,
                    contribution: like_contrib,
                },
                FactorContribution {
                    name: "Thời gian (giờ)".to_string(),
                    raw_value: age_hours,
                    weight: config.gravity,
                    contribution: -decay,
                },
            ],
        })
    }

    pub fn compute_heuristic_score(
        text: &str,
        depth: u32,
        is_author_reply: bool,
        config: &RankingConfig,
    ) -> ScoreBreakdown {
        // len_norm: normalize length up to 500 characters
        let char_count = text.chars().count() as f64;
        let len_norm = (char_count / 500.0).min(1.0);

        // keyword hits
        let lower = text.to_lowercase();
        let hits = config
            .keywords
            .iter()
            .filter(|kw| lower.contains(kw.as_str()))
            .count() as f64;
        let kw_norm = (hits / 3.0).min(1.0);

        // depth bonus: depth 1 gets highest, diminishes slowly
        let depth_bonus = if depth > 0 {
            (1.0 / depth as f64).min(1.0)
        } else {
            1.0
        };

        let author_val = if is_author_reply { 1.0 } else { 0.0 };

        let len_contrib = len_norm * config.weight_len;
        let kw_contrib = kw_norm * config.weight_keywords;
        let depth_contrib = depth_bonus * config.weight_depth;
        let author_contrib = author_val * config.weight_author_reply;

        let total = len_contrib + kw_contrib + depth_contrib + author_contrib;

        ScoreBreakdown {
            total_score: total,
            kind: ScoreKind::Heuristic,
            label: "◌ Ước lượng".to_string(),
            factors: vec![
                FactorContribution {
                    name: "Độ dài văn bản".to_string(),
                    raw_value: char_count,
                    weight: config.weight_len,
                    contribution: len_contrib,
                },
                FactorContribution {
                    name: "Từ khóa liên quan".to_string(),
                    raw_value: hits,
                    weight: config.weight_keywords,
                    contribution: kw_contrib,
                },
                FactorContribution {
                    name: "Vị trí trong luồng".to_string(),
                    raw_value: depth as f64,
                    weight: config.weight_depth,
                    contribution: depth_contrib,
                },
                FactorContribution {
                    name: "Phản hồi của tác giả".to_string(),
                    raw_value: author_val,
                    weight: config.weight_author_reply,
                    contribution: author_contrib,
                },
            ],
        }
    }

    pub fn score_item(
        reply_count: Option<i64>,
        like_count: Option<i64>,
        created_at: i64,
        now: i64,
        text: &str,
        depth: u32,
        is_author_reply: bool,
        config: &RankingConfig,
    ) -> ScoreBreakdown {
        if let Some(real) =
            Self::compute_real_score(reply_count, like_count, created_at, now, config)
        {
            real
        } else {
            Self::compute_heuristic_score(text, depth, is_author_reply, config)
        }
    }
}
