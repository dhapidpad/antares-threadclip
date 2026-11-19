# ADR 0004: Mô hình Thời gian Frame Số nguyên và Rational FPS

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19

## Ngữ cảnh
Biểu diễn thời gian bằng số thực float có thể gây ra sai số làm tròn tích lũy, dẫn đến hiện tượng trôi khung hình hoặc không đồng bộ âm thanh khi render video dài.

## Quyết định
Toàn bộ timeline và kế hoạch render sử dụng đơn vị **frame số nguyên (`u64`)** kết hợp với **tỉ lệ khung hình hữu tỉ (`RationalFps { num, den }`)**. Mọi phép đổi sang milliseconds hoặc giây đều tính toán chính xác và có kiểm thử vòng tròn (round-trip).

## Hệ quả
- Đảm bảo độ chính xác tuyệt đối từng khung hình (frame-accurate).
- Kết quả render deterministic giữa các lần chạy.
