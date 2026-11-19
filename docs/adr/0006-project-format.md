# ADR 0006: Định dạng Dự án Thư mục Tự đủ (Self-contained) và Gói .tcproj

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19

## Ngữ cảnh
Một dự án video bao gồm file định nghĩa timeline, file media (ảnh, meme, nhạc, voice) và bản sao lưu dữ liệu văn bản. Cần định dạng lưu trữ bền vững, không bị gãy link khi di chuyển thư mục hoặc chuyển sang máy khác.

## Quyết định
Mỗi dự án được tổ chức thành một thư mục riêng:
- `project.json`: file cấu trúc timeline có `schema_version`.
- `media/`: chứa toàn bộ file assets đã import (đổi tên theo SHA-256 để chống trùng lặp).
- `cache/`: lưu tạm audio TTS và frame nháp.
- `renders/`: lưu các file MP4 đã xuất.
Tính năng xuất gói dự án `.tcproj` nén zip toàn bộ thư mục này và kiểm tra SHA-256 khi nhập lại.

## Hệ quả
- Dự án độc lập, không phụ thuộc vào đường dẫn tuyệt đối ban đầu của file gốc.
- Đảm bảo khả năng phục hồi và di chuyển dễ dàng giữa các hệ điều hành.
