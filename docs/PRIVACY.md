# Chính sách Bảo vệ Quyền riêng tư (Privacy Policy)

**Antares ThreadClip Studio** được thiết kế theo nguyên tắc **Local-first** và **Privacy-by-Design**.

## 1. Dữ liệu lưu trữ cục bộ
- Toàn bộ danh sách bài viết, bình luận, dự án, assets và video render được lưu trực tiếp trên thiết bị của người dùng (tại thư mục SQLite `data/`).
- Token truy cập OAuth được lưu trữ an toàn trong keychain của hệ điều hành, không ghi dưới dạng văn bản thường và không bao giờ lưu trong cơ sở dữ liệu.
- Hệ thống cung cấp nút "Xóa tài khoản & dữ liệu" cho phép dọn sạch toàn bộ dữ liệu cục bộ bất kỳ lúc nào.

## 2. Kết nối ra bên ngoài
Ứng dụng chỉ thực hiện kết nối ra ngoài trong các trường hợp được người dùng chủ động cho phép:
1. **Threads Graph API:** Chỉ kết nối tới `graph.threads.net` để tải nội dung của chính tài khoản đã ủy quyền thông qua OAuth. Không quét dữ liệu người khác.
2. **Text-to-Speech (nếu dùng Cloud):** Văn bản cần đọc được gửi tới nhà cung cấp TTS (ví dụ Google Cloud TTS). Nếu người dùng chọn chế độ Offline hoặc Mock TTS, không có dữ liệu nào rời khỏi máy.

## 3. Không Telemetry
Ứng dụng mặc định tắt hoàn toàn telemetry và theo dõi hành vi người dùng.
Mọi bản ghi chẩn đoán xuất ra đều được lọc và che (redaction) token, email và thông tin nhận dạng trước khi lưu.
