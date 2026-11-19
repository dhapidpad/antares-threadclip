# ADR 0003: Chạy FFmpeg dưới dạng Tiến trình Sidecar

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19

## Ngữ cảnh
Render video đòi hỏi công cụ mạnh mẽ hỗ trợ đa dạng bộ lọc và codec. Việc liên kết tĩnh (static linking) thư viện C/C++ có thể gây rủi ro về giấy phép (GPL/LGPL) và phức tạp khi đóng gói đa hệ điều hành.

## Quyết định
Thực thi FFmpeg dưới dạng **tiến trình độc lập (sidecar process)**. Lệnh gọi truyền đối số dạng mảng (vector of arguments), không sử dụng shell để phòng chống lỗ hổng chèn lệnh.

## Hệ quả
- Tách biệt ranh giới tiến trình: crash trong FFmpeg không làm sập ứng dụng chính.
- Tuân thủ giấy phép phân phối phần mềm an toàn.
