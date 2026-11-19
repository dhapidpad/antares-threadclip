# Antares ThreadClip Studio

> Desktop Creator Studio **local-first** biến post & replies Threads của chính người dùng thành video ngắn MP4 frame-accurate chuẩn 9:16 (Shorts/TikTok/Reels).

[![CI](https://github.com/dhapidpad/antares-threadclip/actions/workflows/ci.yml/badge.svg)](https://github.com/dhapidpad/antares-threadclip/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## Tính năng chính
- 🔒 **Local-first & Bảo mật:** Toàn bộ dữ liệu bài viết lưu trên SQLite cục bộ. Không có server trung gian, không rò rỉ token.
- 🌳 **Nhập dữ liệu Threads thông minh:** Tự động tái dựng cây thảo luận (thread tree) đa cấp, tính toán độ sâu (`depth`) và phân biệt bình luận tác giả (`is_author_reply`). Hỗ trợ nhập offline từ JSON.
- 🔍 **Tìm kiếm FTS5 & Lọc đa chiều:** Full-text search không dấu (Unicode61 diacritics removal), lọc theo tác giả, thời gian, số lượng phản hồi, nhãn.
- 📊 **Xếp hạng minh bạch:** Tách bạch rõ ràng giữa điểm thực tế (`Real-score` từ engagement) và điểm ước lượng (`Heuristic-score`).
- 🎬 **Timeline đa track & Frame-accurate:** Quản lý thời gian theo số frame nguyên và tỉ lệ khung hình chuẩn hữu tỉ, hỗ trợ undo/redo không giới hạn.
- 🎙️ **Âm thanh & TTS tiếng Việt:** Tích hợp bộ tạo giọng đọc, xuất phụ đề SRT/WebVTT tự động, áp dụng audio ducking tự động hạ âm lượng nhạc nền khi có lời thoại.
- 🎞️ **Render MP4 chuẩn xác:** Xuất video qua FFmpeg với filter-complex tối ưu, đảm bảo tính xác định (determinism) qua `plan_hash`.

## Cấu trúc Workspace
```text
antares-threadclip/
├── crates/
│   ├── tc-core/       # Kiểu miền, tính toán frame/thời gian, mã lỗi
│   ├── tc-threads/    # OAuth loopback, rate limiter, bộ dựng cây thread
│   ├── tc-db/         # SQLite schema v1, FTS5, migrations, repositories
│   ├── tc-filter/     # Trình biên dịch AST truy vấn sang SQL & FTS5
│   ├── tc-ranking/    # Thuật toán tính điểm Real & Heuristic minh bạch
│   ├── tc-timeline/   # Mô hình project, timeline tracks & undo/redo
│   ├── tc-audio/      # TTS provider, captioning, ducking & loudness
│   ├── tc-assets/     # Quản lý tài nguyên media, kiểm tra giấy phép CC0/CC-BY
│   ├── tc-render/     # Bộ lập kế hoạch render & điều khiển FFmpeg sidecar
│   └── tc-app/        # Bộ lệnh IPC Tauri, cấu hình và ghi log che dữ liệu
├── ui/                # Giao diện Vite + React + TypeScript + Tailwind CSS
├── docs/              # Tài liệu ADR, verification, spec và QA
└── testdata/          # Bộ dữ liệu mẫu fixture & golden test
```

## Yêu cầu môi trường
- Rust 1.80+ (khuyên dùng stable)
- Node.js 20+ & npm
- FFmpeg (được cài đặt trong PATH hệ thống)

## Hướng dẫn cài đặt & Chạy thử
```bash
# Kiểm tra và chạy kiểm thử toàn bộ workspace Rust
cargo test --workspace

# Cài đặt phụ thuộc và chạy giao diện UI
cd ui
npm install
npm run build
```

## Giấy phép
Dự án được phân phối dưới giấy phép MIT hoặc Apache-2.0.
