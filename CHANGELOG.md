# Changelog

Tất cả những thay đổi đáng chú ý của dự án Antares ThreadClip Studio sẽ được ghi lại trong tệp này.
Định dạng dựa trên [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) và tuân thủ [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2026-11-19

### Khởi tạo & Kiến trúc (M0 & M1)
- **feat(core):** Khởi tạo `tc-core` với mô hình frame nguyên, FPS hữu tỉ `RationalFps`, Timecode và mã lỗi chuẩn `TcError`.
- **feat(db):** Thiết lập `tc-db` trên SQLite với chế độ WAL, FTS5 hỗ trợ tiếng Việt không dấu (unicode61), sequential migrations và cơ chế backup/rollback.
- **feat(threads):** Triển khai `tc-threads` bao gồm token bucket rate limiter, bộ dựng cây thảo luận `TreeBuilder` tính độ sâu và cờ tác giả, cùng bộ nhập offline từ JSON.
- **feat(filter):** Xây dựng `tc-filter` biên dịch AST truy vấn JSON sang câu lệnh SQL tham số hóa an toàn chống SQL injection.
- **feat(ranking):** Hiện thực thuật toán `tc-ranking` tách biệt minh bạch giữa Real-score và Heuristic-score kèm bảng giải thích đóng góp điểm số.
- **feat(timeline):** Mô hình hóa timeline đa track `tc-timeline` với ngăn xếp undo/redo không giới hạn theo Command pattern.
- **feat(audio):** Triển khai trait `TtsProvider`, `MockTtsProvider` sinh file WAV hợp lệ, bộ xuất phụ đề SRT/WebVTT và graph ducking âm lượng -12dB qua FFmpeg sidechain.
- **feat(assets):** Quản lý tài nguyên media `tc-assets`, kiểm soát giấy phép bản quyền CC0/CC-BY và tự động sinh tệp `credits.txt`.
- **feat(render):** Xây dựng `tc-render` tính toán `plan_hash` đảm bảo tính xác định (determinism) và điều khiển tiến trình FFmpeg sidecar xuất video MP4.
- **feat(app):** Tích hợp `tc-app` kết nối toàn bộ hệ thống với bộ lọc che dữ liệu nhạy cảm (redaction).
- **feat(ui):** Phát triển giao diện React + Vite + TypeScript với đầy đủ 11 màn hình S01-S11 và hỗ trợ đa ngôn ngữ vi/en.
- **ci:** Cấu hình GitHub Actions CI đa nền tảng (Ubuntu, Windows, macOS) kiểm thử tự động, linting và e2e render.
- **docs:** Hoàn thiện 6 bản ADR (0001-0006), sổ xác minh V-01..V-11, ma trận TTS và tài liệu chính sách bảo mật riêng tư.
