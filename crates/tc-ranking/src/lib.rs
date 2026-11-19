pub mod config;
pub mod score;

pub use config::RankingConfig;
pub use score::{FactorContribution, RankingEngine, ScoreBreakdown, ScoreKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_score_computation() {
        let config = RankingConfig::default();
        let now = 10000;
        let created_at = 10000 - 3600; // 1 hour ago

        let breakdown =
            RankingEngine::compute_real_score(Some(10), Some(20), created_at, now, &config)
                .unwrap();

        assert_eq!(breakdown.kind, ScoreKind::Real);
        assert!(breakdown.total_score > 0.0);
        assert_eq!(breakdown.label, "● Dữ liệu thật");
    }

    #[test]
    fn test_heuristic_score_fallback() {
        let config = RankingConfig::default();
        let text = "Ứng dụng AI sáng tạo video rất hay và tiện lợi";

        let breakdown = RankingEngine::compute_heuristic_score(text, 1, true, &config);
        assert_eq!(breakdown.kind, ScoreKind::Heuristic);
        assert!(breakdown.total_score > 0.0);
        assert_eq!(breakdown.label, "◌ Ước lượng");
    }
}
