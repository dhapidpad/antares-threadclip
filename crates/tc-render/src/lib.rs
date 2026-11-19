pub mod ffmpeg;
pub mod golden;
pub mod plan;

pub use ffmpeg::FFmpegCommandBuilder;
pub use golden::GoldenTestSuite;
pub use plan::{RenderPlan, RenderPreset};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_render_plan_hash_determinism() {
        let proj = GoldenTestSuite::create_sample_text_only_project();
        let preset = RenderPreset::shorts_1080x1920();

        let plan1 = RenderPlan::new(proj.clone(), preset.clone());
        let plan2 = RenderPlan::new(proj, preset);

        assert_eq!(plan1.plan_hash, plan2.plan_hash);
        assert!(!plan1.plan_hash.is_empty());
    }

    #[test]
    fn test_ffmpeg_args_generation() {
        let proj = GoldenTestSuite::create_sample_text_only_project();
        let preset = RenderPreset::draft_540x960();
        let plan = RenderPlan::new(proj, preset);

        let out = Path::new("test_out.mp4");
        let args = FFmpegCommandBuilder::build_args(&plan, None, None, None, out);

        assert!(args.contains(&"-filter_complex".to_string()));
        assert!(args.contains(&"libx264".to_string()));
        assert!(args.contains(&"yuv420p".to_string()));
    }
}
