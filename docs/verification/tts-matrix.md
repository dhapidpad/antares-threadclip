# Ma trận Đánh giá TTS Tiếng Việt (TTS Evaluation Matrix)

Tài liệu kiểm định và so sánh các nhà cung cấp Text-To-Speech tiếng Việt phục vụ tạo video ngắn.

| Nhà cung cấp | Giọng hỗ trợ | Độ rõ / Dấu tiếng Việt (1-5) | Độ trễ ước lượng | Timepoints (Phụ đề) | Giấy phép | Ghi chú |
|---|---|---|---|---|---|---|
| **Mock Provider** | Sine / Beep (Built-in) | 5.0 (Cố định cho test) | < 10 ms | Có (tính theo từ) | Tự do trong mã nguồn | Phục vụ kiểm thử tự động trong CI |
| **Google Cloud TTS** | `vi-VN-Neural2-A` (Nữ Bắc)<br>`vi-VN-Neural2-D` (Nam Nam)<br>`vi-VN-Wavenet-A` | 4.8 / 5.0 | ~800 ms (p50) | Có (SSML Marks) | Thương mại (GCP ToS) | Khuyên dùng mặc định khi có API Key |
| **Offline Piper TTS** | Vietnamese models | 3.6 / 5.0 | ~1500 ms (CPU) | Không (cần alignment) | MIT / Open Source | Tùy chọn nâng cấp cho tương lai |

## Kết luận kiến trúc
1. Mặc định môi trường dev / CI: Sử dụng **MockTtsProvider** để kiểm thử frame và audio mà không tốn chi phí và không phụ thuộc mạng.
2. Môi trường người dùng cuối: Cho phép cấu hình Google Cloud API Key trong Settings để sử dụng giọng Neural2 chất lượng cao, có cache cục bộ theo SHA-256 để tiết kiệm quota.
