# ADR 0002: SQLite với WAL và FTS5 cho lưu trữ cục bộ

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19

## Ngữ cảnh
Cần lưu trữ bài viết, bình luận dạng cây, nhãn và thông tin dự án cục bộ với khả năng tìm kiếm toàn văn nhanh chóng, an toàn và hỗ trợ giao dịch.

## Quyết định
Sử dụng **SQLite** nhúng qua `rusqlite` kèm tính năng bundled FTS5 (`unicode61 remove_diacritics 2`). Kích hoạt chế độ `journal_mode = WAL` và kiểm tra khóa ngoại `foreign_keys = ON`. Sử dụng migrations tuần tự có lưu `schema_version`.

## Hệ quả
- Khả năng tìm kiếm tiếng Việt không dấu siêu tốc mà không cần dịch vụ tìm kiếm bên ngoài.
- Dễ dàng sao lưu (copy file DB) và rollback khi cần thiết.
