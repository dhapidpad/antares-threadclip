use crate::plan::RenderPlan;
use std::path::{Path, PathBuf};
use tc_core::{Result, TcError};

pub struct FFmpegCommandBuilder;

impl FFmpegCommandBuilder {
    /// Builds the safe argument vector for FFmpeg to render the plan
    pub fn build_args(
        plan: &RenderPlan,
        bg_video_path: Option<&Path>,
        voice_audio_path: Option<&Path>,
        music_audio_path: Option<&Path>,
        output_file: &Path,
    ) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        args.push("-y".to_string()); // overwrite

        let duration_secs = plan.total_frames.to_seconds(plan.preset.fps).max(1.0);
        let mut filter_complex = Vec::new();
        let mut input_idx = 0;

        // 1. Background video or color canvas
        let video_source_tag = match bg_video_path {
            Some(p) => {
                args.push("-stream_loop".to_string());
                args.push("-1".to_string());
                args.push("-i".to_string());
                args.push(p.to_string_lossy().to_string());
                let tag = format!("{}:v", input_idx);
                input_idx += 1;
                tag
            }
            None => {
                // Generate solid color canvas
                args.push("-f".to_string());
                args.push("lavfi".to_string());
                args.push("-i".to_string());
                args.push(format!(
                    "color=c=black:s={}x{}:r={}:d={}",
                    plan.preset.width,
                    plan.preset.height,
                    plan.preset.fps.to_ratio_string(),
                    duration_secs
                ));
                let tag = format!("{}:v", input_idx);
                input_idx += 1;
                tag
            }
        };

        // Scale & crop background to exact preset width/height
        filter_complex.push(format!(
            "[{}]scale={}:{}:force_original_aspect_ratio=increase,crop={}:{}:(in_w-{})/2:(in_h-{})/2,fps={}[v_bg]",
            video_source_tag,
            plan.preset.width,
            plan.preset.height,
            plan.preset.width,
            plan.preset.height,
            plan.preset.width,
            plan.preset.height,
            plan.preset.fps.as_f64()
        ));

        // 2. Audio inputs
        let mut has_audio = false;
        let mut voice_tag = None;
        if let Some(vp) = voice_audio_path {
            args.push("-i".to_string());
            args.push(vp.to_string_lossy().to_string());
            voice_tag = Some(format!("{}:a", input_idx));
            input_idx += 1;
            has_audio = true;
        }

        let mut music_tag = None;
        if let Some(mp) = music_audio_path {
            args.push("-i".to_string());
            args.push(mp.to_string_lossy().to_string());
            music_tag = Some(format!("{}:a", input_idx));
            input_idx += 1;
            has_audio = true;
        }

        let audio_out_tag = if let (Some(v), Some(m)) = (voice_tag, music_tag) {
            filter_complex.push(tc_audio::DuckingFilterBuilder::build_ducking_filter(
                &m, &v, "a_out",
            ));
            Some("a_out")
        } else if let Some(v) = voice_tag {
            filter_complex.push(tc_audio::DuckingFilterBuilder::build_voice_only_filter(
                &v, "a_out",
            ));
            Some("a_out")
        } else if let Some(m) = music_tag {
            filter_complex.push(format!(
                "[{}]aresample=48000,loudnorm=I=-14:TP=-1:LRA=11[a_out]",
                m
            ));
            Some("a_out")
        } else {
            None
        };

        // Add duration limit
        args.push("-t".to_string());
        args.push(format!("{:.3}", duration_secs));

        // Filter complex
        args.push("-filter_complex".to_string());
        args.push(filter_complex.join(";"));

        // Video mapping & encoding
        args.push("-map".to_string());
        args.push("[v_bg]".to_string());
        args.push("-c:v".to_string());
        args.push("libx264".to_string());
        args.push("-preset".to_string());
        args.push(plan.preset.preset_speed.clone());
        args.push("-crf".to_string());
        args.push(plan.preset.crf.to_string());
        args.push("-pix_fmt".to_string());
        args.push("yuv420p".to_string());
        args.push("-movflags".to_string());
        args.push("+faststart".to_string());

        // Audio mapping & encoding
        if let Some(a_tag) = audio_out_tag {
            args.push("-map".to_string());
            args.push(format!("[{}]", a_tag));
            args.push("-c:a".to_string());
            args.push("aac".to_string());
            args.push("-b:a".to_string());
            args.push(plan.preset.audio_bitrate.clone());
        } else {
            args.push("-an".to_string());
        }

        args.push(output_file.to_string_lossy().to_string());
        args
    }

    /// Renders video via FFmpeg executable sidecar
    pub fn execute_render(ffmpeg_bin: &str, args: &[String]) -> Result<()> {
        let output = std::process::Command::new(ffmpeg_bin)
            .args(args)
            .output()
            .map_err(|e| TcError::RenderError(format!("Failed to start FFmpeg: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TcError::RenderError(format!("FFmpeg failed: {stderr}")));
        }

        Ok(())
    }
}
