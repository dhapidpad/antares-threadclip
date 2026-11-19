# ADR 0005: Lưu trữ Bí mật trong OS Keychain thay vì Plaintext

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19

## Ngữ cảnh
Token dài hạn của Threads và API keys của dịch vụ TTS là các thông tin nhạy cảm. Việc lưu vào file config text hoặc trong DB SQLite có nguy cơ bị rò rỉ khi người dùng chia sẻ thư mục dự án hoặc file log.

## Quyết định
Sử dụng **OS Keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)** qua crate `keyring`. Cơ sở dữ liệu SQLite chỉ lưu `token_expires_at` (epoch s) để cảnh báo hạn dùng, tuyệt đối không lưu token.

## Hệ quả
- Tuân thủ nguyên tắc GR-02 (Privacy-first) và GR-03 (No secrets in storage/repo).
- An toàn trước các vụ tấn công đánh cắp file dữ liệu.
