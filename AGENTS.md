# Antares ThreadClip Studio – Agent Protocol

> Quy ước làm việc bắt buộc dành cho AI coding agents tham gia dự án. Trỏ trực tiếp từ tài liệu `docs/spec/ANTARES_THREAD_CLIP_STUDIO_PRODUCTION_SPEC_V3.md`.

## 1. Nguyên tắc cốt lõi (Guardrails bất biến)
- **GR-01 (Local-first):** Dữ liệu gốc lưu trên máy; chỉ gọi API đã được ủy quyền.
- **GR-02 (Privacy-first):** Không log token/email/nội dung riêng tư; không gửi dữ liệu ra ngoài khi chưa có opt-in.
- **GR-03 (No secrets):** Không commit secrets, token hoặc credential vào repo.
- **GR-04 (No voice cloning):** Không clone giọng nói khi không có sự đồng thuận; chỉ dùng giọng có sẵn từ nhà cung cấp.
- **GR-05 (License-aware):** Mọi asset phải có thông tin bản quyền rõ ràng (CC0, CC-BY, PD, USER-OWNED).
- **GR-06 (No falsified data):** Không bịa số liệu engagement hay giả mạo timestamp mà không gắn nhãn cờ cảnh báo.
- **GR-07 (Integrity):** Không force-push, không bỏ qua test lỗi, không tuyên bố thành công thiếu bằng chứng.
- **GR-08 (Permissions):** Thao tác push/PR/release phải có sự cho phép rõ ràng từ người dùng.

## 2. Quy trình cho mỗi Task
1. Đọc task ID, acceptance criteria, và bằng chứng cần nộp.
2. Kiểm tra `git status` sạch sẽ.
3. Viết test và mã nguồn tối thiểu đạt chuẩn.
4. Chạy kiểm tra chất lượng: `cargo fmt`, `cargo clippy`, `cargo test`, `npm test` (trong `ui`).
5. Tự review diff, cập nhật docs/ADR nếu thay đổi kiến trúc.
6. Commit theo chuẩn Conventional Commits (`feat(...)`, `fix(...)`, `test(...)`, `docs(...)`).
7. Tạo báo cáo QA trong `docs/qa/<task-id>.md`.
8. Push lên GitHub khi được cấp quyền.

## 3. Cấp quyền Agent
- **L1:** Sửa code cục bộ, chạy test, commit cục bộ trên nhánh riêng.
- **L2:** Cho phép push nhánh và mở Pull Request lên GitHub (khi người dùng cho phép).
- **L3:** Tạo tag / release.
