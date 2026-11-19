# ADR 0001: Lựa chọn Tech Stack - Tauri 2 + Rust + React/Vite

- **Trạng thái:** Accepted
- **Ngày:** 2026-11-19
- **Tác giả:** Antares Studio Architect

## Ngữ cảnh
Ứng dụng Antares ThreadClip Studio là một Desktop Creator Studio cần hiệu năng cao, xử lý video chính xác từng khung hình và giao diện trực quan, linh hoạt.

## Quyết định
Sử dụng **Rust** cho toàn bộ core logic (database, timeline, audio, render planning, thread tree) và **Tauri 2** kết hợp **React / TypeScript / Vite** cho giao diện người dùng.

## Hệ quả
- Nhẹ nhàng, tiết kiệm RAM hơn so với Electron (~400MB target).
- Tận dụng hệ sinh thái UI phong phú của web (canvas, timeline visualizer) kết hợp tính an toàn và tốc độ của Rust.
