use tc_app::{AppController, AppState};
use tc_audio::{CaptionGenerator, MockTtsProvider, TtsProvider};
use tc_core::{Frame, RationalFps, Timecode};
use tc_db::Database;
use tc_ranking::{RankingConfig, RankingEngine};
use tc_render::{FFmpegCommandBuilder, RenderPlan, RenderPreset};
use tc_timeline::{Clip, ClipContent, ProjectModel};

#[test]
fn test_user_threads_post_workflow() {
    // 1. Initialize in-memory AppState
    let db = Database::open_in_memory().expect("Failed to open DB");
    let state = AppState::new(db);

    // 2. Load the exact user's Threads post fixture: Df5D-Hak-d3
    let json_data = include_str!("../../../testdata/test_user_thread_Df5D.json");
    let (posts_imported, replies_imported) =
        AppController::import_offline_json(&state, json_data).expect("Import must succeed");

    assert_eq!(posts_imported, 1);
    assert_eq!(replies_imported, 14);

    // 3. Verify Thread Tree reconstruction and depth calculation
    let post_id = "Df5D-Hak-d3";
    let tree = AppController::get_post_tree(&state, post_id).expect("Tree build must succeed");

    assert_eq!(tree.post.id.0, post_id);
    assert_eq!(tree.post.author, "cung_khong_iu_toan_lam");
    assert_eq!(tree.post.text, "như này có nên chan không mng ??");
    assert_eq!(tree.post.like_count, Some(602));
    assert_eq!(tree.post.reply_count, Some(34));

    // Verify author replies have is_author_reply = true
    let author_replies: Vec<_> = tree.replies.iter().filter(|r| r.is_author_reply).collect();
    assert_eq!(author_replies.len(), 6);
    for r in &author_replies {
        assert_eq!(r.author, "cung_khong_iu_toan_lam");
    }

    // Verify 4-level deep conversation thread
    let deep_reply = tree
        .replies
        .iter()
        .find(|r| r.id.0 == "Df6Q50ik5ur")
        .expect("Level 4 author reply must exist");
    assert_eq!(deep_reply.depth, 4);
    assert_eq!(deep_reply.text, "sợ ngta dỗi b ạ 😔🥀");
    assert!(deep_reply.is_author_reply);

    // 4. Test Dual-Ranking Engine (RealScore & HeuristicScore)
    let score = AppController::score_post(&state, post_id).expect("Score must succeed");
    assert!(score.real_score > 0.0);
    assert!(score.heuristic_score > 0.0);
    assert!(score.final_score > 0.0);

    // Verify reply scoring - author reply receives author bonus
    let cfg = RankingConfig::default();
    let now = chrono::Utc::now().timestamp();
    let rep_score = RankingEngine::score_item(
        deep_reply.reply_count,
        deep_reply.like_count,
        deep_reply.created_at,
        now,
        &deep_reply.text,
        deep_reply.depth,
        deep_reply.is_author_reply,
        &cfg,
    );
    assert!(rep_score.breakdown.contains_key("author_reply_bonus"));

    // 5. Timeline Editor Setup: Create 9:16 Shorts Project
    let mut proj = ProjectModel::default();
    let fps = RationalFps::fps_30();

    let root_clip = Clip {
        id: "clip_root".to_string(),
        start_frame: Frame(0),
        duration_frames: Frame(90), // 3.0s
        content: ClipContent::Text {
            body: tree.post.text.clone(),
            author: tree.post.author.clone(),
        },
    };
    proj.tracks[0].clips.push(root_clip);

    let reply_clip = Clip {
        id: "clip_rep_deep".to_string(),
        start_frame: Frame(90),
        duration_frames: Frame(90), // 3.0s
        content: ClipContent::Text {
            body: deep_reply.text.clone(),
            author: deep_reply.author.clone(),
        },
    };
    proj.tracks[0].clips.push(reply_clip);

    let duration_tc = proj.total_duration_timecode();
    assert_eq!(duration_tc, "00:00:06:00"); // 180 frames @ 30fps = 6.0s

    // 6. Generate TTS Audio and SRT Subtitles
    let tts = MockTtsProvider::new();
    let audio_wav = tts
        .synthesize(&tree.post.text, "vi-VN-Standard-A", 1.0)
        .expect("TTS synthesis should succeed");
    assert!(!audio_wav.is_empty());

    let captions = vec![
        (
            Timecode::from_frames(Frame(0), fps),
            Timecode::from_frames(Frame(90), fps),
            tree.post.text.clone(),
        ),
        (
            Timecode::from_frames(Frame(90), fps),
            Timecode::from_frames(Frame(180), fps),
            deep_reply.text.clone(),
        ),
    ];
    let srt = CaptionGenerator::to_srt(&captions);
    assert!(srt.contains("như này có nên chan không mng ??"));
    assert!(srt.contains(&deep_reply.text));

    // 7. Render Engine: Build Deterministic RenderPlan and FFmpeg Command
    let preset = RenderPreset::shorts_1080x1920();
    let plan = RenderPlan::new(proj, preset);

    assert!(!plan.plan_hash.is_empty());
    assert_eq!(plan.total_frames, 180);

    let out_file = std::path::Path::new("output_shorts.mp4");
    let ffmpeg_args = FFmpegCommandBuilder::build_args(&plan, None, None, None, out_file);

    assert!(ffmpeg_args.contains(&"-filter_complex".to_string()));
    assert!(ffmpeg_args.contains(&"1080".to_string()));
    assert!(ffmpeg_args.contains(&"1920".to_string()));
    assert!(ffmpeg_args.contains(&"libx264".to_string()));
    assert!(ffmpeg_args.contains(&"output_shorts.mp4".to_string()));
}
