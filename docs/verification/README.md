# Verification Register (Sổ Xác Minh)

Danh sách kết quả kiểm chứng cho các giả định kỹ thuật theo mục 2 của đặc tả Antares ThreadClip Studio V3.

| ID | Giả định cần kiểm chứng | Trạng thái | Phương án Fallback |
|---|---|---|---|
| V-01 | Threads API trả replies của post qua `GET /{media-id}/conversation` kèm `replied_to` | PARTIAL | Dựng cây theo cấp từ `/replies` hoặc nhập JSON offline |
| V-02 | Có thể lấy `like_count`, `reply_count` cho bài viết của mình | PARTIAL | Chuyển sang chế độ Heuristic-score khi metrics là NULL |
| V-03 | Hạn mức đọc/ghi thực tế của Threads API | VERIFIED | Token bucket cục bộ cấu hình được, mặc định 20 burst, 0.055 call/s |
| V-04 | `threads_publishing_limit` chỉ đo hạn mức đăng | VERIFIED | Bộ đếm request cục bộ độc lập với endpoint đăng |
| V-05 | Token long-lived cần refresh trước 60 ngày | VERIFIED | Cảnh báo UI khi còn <= 7 ngày, tự refresh khi còn <= 14 ngày |
| V-06 | OAuth loopback desktop (`127.0.0.1:<random-port>`) | VERIFIED | Luồng nhập thủ công JSON / paste văn bản offline |
| V-07 | Google Cloud TTS có giọng `vi-VN` và trả timepoints | VERIFIED | Gắn cờ `timing_estimated = true` khi ước lượng theo từ |
| V-08 | TTS Offline tiếng Việt chất lượng chấp nhận được | UNVERIFIED | Cho phép chạy video không TTS (text + nhạc nền) |
| V-09 | FFmpeg sidecar frame-accurate + ducking + loudnorm | VERIFIED | Bộ kiểm thử Golden e2e so sánh hash khung hình mốc |
| V-10 | Gemini 3.8 Flash TTS tính khả dụng | REMOVED | Bỏ khỏi phạm vi v3; dùng provider Google Cloud / Mock |
| V-11 | Tauri 2 + Canvas đủ 60 fps cho timeline | VERIFIED | Canvas 2D tối ưu hóa và ảo hóa danh sách bài viết |
