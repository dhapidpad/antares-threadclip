# ANTARES_THREAD_CLIP_STUDIO_PRODUCTION_SPEC_V3.md

Bản đặc tả sản phẩm Antares ThreadClip Studio V3 là phiên bản nâng cấp từ bản V2, tiếp tục mục tiêu xây dựng một **Desktop Creator Studio** “local-first” cho Threads (bao gồm posts và comments), với khả năng lọc tìm kiếm và lựa chọn nội dung mạnh mẽ, cùng giao diện người dùng cao cấp cho việc import, xem xét, chỉnh sửa, timeline, quản lý tài nguyên, lồng tiếng/meme, và xuất video short-form đầu ra chính xác từng khung hình (frame-accurate MP4). V3 bổ sung các thành phần quan trọng thiếu ở V2: dữ liệu/schema versioning, ranh giới module, ADR (architecture decision records), luồng UX màn hình chi tiết, phân loại lọc và mô hình truy vấn, thuật toán xếp hạng bình luận minh bạch (tách dữ liệu thực và heuristic), quy trình việc của người tạo (creator workflow), chỉ đạo chuyển động (motion direction), gói meme nguyên bản, ma trận đánh giá TTS tiếng Việt, hiệu năng/determinism rendering, lưu trữ/cache/phục hồi, tính khả dụng (accessibility)/đa ngôn ngữ, xử lý lỗi/giám sát, mô hình đe dọa/riêng tư/tuân thủ license, lộ trình ưu tiên với tiêu chí nghiệm thu đo lường. Đặc biệt, V3 thiết kế **tự động hóa GitHub end-to-end**: quy định kiểm tra repo, chiến lược nhánh an toàn, commit theo chuẩn Conventional Commits, workflow Pull Request, GitHub Actions CI (format, lint, typecheck, unit/integration/e2e, build, package) trên đa nền tảng, bộ nhớ đệm phụ thuộc, release, SBOM/signing (nếu khả thi), quản lý secrets, bảo vệ nhánh, cổng kiểm thử, rollback, đánh số phiên bản SemVer, xác minh từng commit. Quy trình này phải cho phép agent sửa code, chạy test/build, kiểm tra ứng dụng, render video mẫu, tạo báo cáo QA, nhưng **chỉ push/PR/release khi có quyền rõ ràng** (không force-push, không đẩy secrets, không bỏ qua lỗi test, không công bố thành công thiếu minh chứng).

Tất cả các yêu cầu trên dựa trên tài liệu nền V2 kèm theo (Antares_ThreadClip_Production_Spec_V2.md), đồng thời kết hợp tham khảo công nghệ hiện hành và nguyên tắc tốt nhất (ví dụ Threads API, TTS, Docker, FFmpeg, GitHub Actions,…). Mỗi phần dưới đây sẽ chỉ rõ chi tiết kỹ thuật để coding agent có thể thực hiện và kiểm chứng từng chức năng một cách chặt chẽ. Các phép tham khảo từ tài liệu và nguồn chính thức được ký hiệu như ****.

## 1. Tổng quan và mục tiêu

**Mục tiêu sản phẩm:** Antares ThreadClip Studio là ứng dụng desktop để **tạo video short-form** từ nội dung Threads (hoặc các nền tảng tương tự) theo cách **local-first** (mọi dữ liệu gốc lưu trên máy cục bộ, chỉ kết nối API được ủy quyền, tôn trọng riêng tư người dùng và hạn chế của API). Các tính năng chính bao gồm:
- **Nhập & tìm kiếm nội dung:** Tải các post và comment của người dùng từ Threads API (hoặc API tương ứng) sau khi xác thực, lưu trữ cục bộ trong cơ sở dữ liệu. Hỗ trợ tìm kiếm, sắp xếp, lọc nâng cao (theo từ khóa, thời gian, tác giả, chỉ số tương tác, cảm xúc, v.v) để chọn nội dung phù hợp.
- **Quản lý lựa chọn:** Xây dựng “thread tree” cho các post (API trả về cấu trúc phẳng kèm `replied_to` để tái tạo cây replies), cho phép chọn từng phần (bài gốc, bình luận, phản hồi) và thực hiện các thao tác hàng loạt (ví dụ gắn thẻ, đánh giá, xóa bỏ selection).
- **Editor video timeline:** Giao diện canvas/timeline để ghép nội dung đã chọn thành video, với các lớp (layers) như nền (background video/gameplay/static), thẻ (cards) hiển thị văn bản, meme/reaction (hình ảnh, sticker) hợp lệ bản quyền, hiệu ứng chuyển cảnh (transitions) và âm thanh.
- **Âm thanh và lồng tiếng:** Tích hợp Text-To-Speech (TTS) tiếng Việt đa giọng (cho phép map nhiều “speaker” khác nhau) hoặc thu âm giọng nói để thuyết minh. Tự động trích xuất caption đồng bộ (SRT/WebVTT) từ lời thoại, hỗ trợ chỉnh sửa. Hỗ trợ audio ducking (tự động giảm âm lượng nhạc nền khi có lời thoại) và trộn nhiều track audio (nhạc nền, SFX, voice).
- **Render chính xác:** Cơ chế xử lý video sử dụng Rust và thư viện đồ họa/phần mềm (ví dụ FFmpeg hay GStreamer bindings) cho phép xuất file MP4 hoàn chỉnh, chuẩn h.264 (hoặc tương đương), với độ chính xác từng khung hình, không để lệch khung hoặc mất đồng bộ. Cho phép xem trước (preview) ngay trên app.
- **Chất lượng UI/UX cao cấp:** Giao diện trực quan, responsive, tối ưu cho người sáng tạo. Các màn hình import (chọn nguồn dữ liệu, login OAuth), review (xem danh sách posts/comments đã tải), editor timeline (kéo thả nội dung lên timeline), quản lý assets (media, meme, fonts, voice config), xuất bản (render queue, lịch sử dự án). Hỗ trợ dark mode, keyboard shortcuts, undo/redo, clipboard.
- **Bảo mật và tuân thủ:** Hoạt động chỉ trên dữ liệu được phép (không crawl vượt giới hạn API, không lấy nội dung riêng tư người khác, không clone giọng thật không có consent). Tôn trọng license/provenance của media (chỉ dùng content CC0/CC-BY hay sưu tầm hợp pháp). Không giả mạo thông tin metrics (khi render phụ đề/voice). Không hardcode secrets, log không rò rỉ dữ liệu người dùng.

Yêu cầu V3 bổ sung như trên (đều dựa vào các gợi ý trong V2) đồng thời tương thích với các “guardrails” đã có: **local-first**, **privacy-first**, không khoan nhượng đạo đức (không copy giọng, không scrap nội dung), kiểm chứng công nghệ, API hiện hành.  

*(Tham khảo: Threads API được Meta cung cấp trên `graph.threads.net/.com`, tách biệt khỏi Instagram Graph API; API này giới hạn chỉ cho phép đọc bài của chính người dùng và replies của nó, không truy cập được dữ liệu chung của người khác.)*

## 2. Kiến trúc tổng quát và mô-đun

**2.1 Phân chia mô-đun chính:** Ứng dụng nên chia thành các module độc lập sau, để rõ ràng ranh giới và dễ bảo trì:

- **Module `threads_api`**: Quản lý tương tác với Threads API (và có thể mở rộng cho các API mạng xã hội khác nếu cần). Chịu trách nhiệm về OAuth, gọi REST (Graph API) để lấy bài gốc và replies. Xử lý giới hạn API (rate limits/quota), refresh token định kỳ (lưu ý Threads yêu cầu refresh 60 ngày một lần).
- **Module `database`**: Lưu trữ cục bộ posts, replies, project data. Sử dụng SQLite hoặc RocksDB qua thư viện Rust tương ứng (ví dụ Diesel + SQLite). Cấu trúc schema versioned, theo semver, và migrations (chú ý crate diesel_migrations tự track phiên bản bảng).
- **Module `filter`**: Định nghĩa mô hình truy vấn và lọc. Nhận input từ UI (từ khóa, thời gian, tác giả, thẻ tag, sentiment, chỉ số tương tác v.v.) và chuyển thành biểu thức lọc (có thể dùng filter builder hoặc tích hợp full-text search). Nếu dữ liệu lớn, cân nhắc dùng chỉ mục tìm kiếm (Ví dụ, tích hợp Tantivy hay SQLite FTS).
- **Module `ranking`**: Đánh giá độ “khả quan trọng” của comments/posts. Nếu có số liệu thực (like/upvotes/reply count), dùng thuật toán kiểu Hotness (ví dụ như Reddit r/x/u^p / (t+2)^something) hoặc HN (Donald Knuth) để xếp hạng. Nếu không, dùng heuristic (độ dài văn bản, xuất hiện keyword, sentiment, độ sâu thread). Phân biệt rõ “xếp hạng dựa trên dữ liệu thật” và “heuristic fallback”.
- **Module `timeline`**: Chịu trách nhiệm vẽ timeline video và xử lý các layer, transitions. Các đối tượng trong timeline: background clip, text card, image meme, audio tracks, v.v. Dữ liệu timeline được lưu trong project database.
- **Module `renderer`**: Kết xuất video cuối cùng. Ví dụ đóng gói FFmpeg hoặc thư viện media (như GStreamer) để trộn các track hình/âm thanh. Đảm bảo định dạng MP4 chuẩn, kiểm soát codec, frame rate, kích thước. Phải đảm bảo render **deterministic trên cùng bộ input** (không phụ thuộc vào cài đặt ngẫu nhiên, dừng chuẩn xác từng khung hình).
- **Module `audio`**: Quản lý âm thanh, TTS. Gồm: tương tác với dịch vụ TTS (Google/AWS/Microsoft/Azure/Coqui), trộn thêm nhạc, SFX, xử lý ducking (tự động giảm nhạc nền khi có voice), và xuất/mix thành track audio hoàn chỉnh.
- **Module `assets`**: Thư viện tài nguyên (meme packs, fonts, nhạc nền). Mỗi asset có metadata (nguồn/lizenz). Hỗ trợ import file (ảnh, video, âm thanh) và quản lý danh mục CC0/royalty-free.
- **Module `ui`**: Giao diện người dùng (Rust + framework GUI, ví dụ Tauri/Electron hay native với egui/GTK). Chứa các thành phần màn hình: đăng nhập, import data, dashboard filter, timeline editor, quản lý dự án, xem trước, render queue.
- **Module `ci_cd`**: Cấu hình tích hợp liên tục, không phải là một module runtime nhưng cần duy trì trong repo: file cấu hình CI (GitHub Actions workflows), và tập các script/command cho lint, build, test.

**2.2 Records quyết định kiến trúc (ADR):** Mỗi quyết định quan trọng cần được ghi lại dưới dạng ADR (Một tập tin Markdown ngắn gọn trong thư mục `doc/adr/`). Mỗi ADR ghi rõ *vấn đề*, *giải pháp được chọn*, *lý do*, và *hệ quả*. Theo khuyến nghị của Martin Fowler, mỗi ADR nên ngắn (1-2 trang) và bao gồm quyết định, ngữ cảnh và hệ quả. ADR phải lưu trong repository code (ví dụ `doc/adr/0001-xxx.md`) để duy trì lịch sử kiến trúc. Không chỉnh sửa ADR sau khi chấp nhận; nếu thay đổi thì tạo ADR mới (đánh số nối tiếp). 

**2.3 API & Tích hợp:**  
- Threads API sử dụng OAuth (client id/secret) để lấy token. Cần cập nhật token trước mỗi 60 ngày (Threads API yêu cầu refresh token 60 ngày/ lần). Tài liệu chỉ rõ token đó sẽ hết hạn nếu không được refresh đúng hạn.  
- Yêu cầu tuân thủ rate limit/quotas: Threads giới hạn 250 bài đăng/ngày, 1000 replies/ngày, 100 deletes/ngày, 500 location search/ngày. Hơn nữa, tổng API calls bị giới hạn 4.800/ngày. Thiết kế module `threads_api` phải kiểm tra, đếm và thông báo khi gần hết hạn ngạch (ví dụ gọi GET `/threads_publishing_limit` để kiểm tra còn bao nhiêu).  
- Threads API **chỉ trả về bài của chính người dùng được cấp quyền**, không có chế độ đọc public feed hoặc post người khác. Do đó, ứng dụng chỉ hỗ trợ lấy threads/posts của tài khoản đã authorize. 
- Điểm đặc biệt: Threads API không hỗ trợ chỉnh sửa (edit) bài đã đăng. Chỉ có thể xóa và tạo lại nếu cần sửa lỗi. Ứng dụng phải hiện cảnh báo cho người dùng rằng “không thể sửa bài cũ, chỉ xóa và đăng lại”.  

## 3. Mô hình dữ liệu và quản lý phiên bản

**3.1 Mô hình miền (Domain Model):**  
- **Post**: bảng lưu các bài viết gốc (Threads). Trường chính: `id`, `text`, `author`, `timestamp`, `metrics` (số reply, like nếu có), `reply_control`, v.v.  
- **Comment/Reply**: bảng lưu bình luận và phản hồi, liên kết đến `post_id` và `replied_to_id` (để tái tạo cấu trúc cây). Bao gồm trường `text`, `author`, `timestamp`, `metrics`, `depth` (có thể tính từ `replied_to`).  
- **Project/Clip**: cấu trúc lưu trữ một video đang tạo: bao gồm danh sách media (các post/comment được chọn), các thiết lập timeline (đối tượng text card, hình ảnh meme, audio track). Có thể chứa `id`, `name`, `created_at`, `settings`.  
- **Asset**: chứa thông tin về meme/gif/sound pack: `id`, `type` (image, audio), `source` (file path hoặc URL), `license_info`.  
- **SpeakerConfig**: cấu hình định nghĩa các giọng TTS đã chọn (mapping từ “speaker name” trong UI đến voice ID trên dịch vụ TTS).

**3.2 Quản lý phiên bản và migrations:**  
- Dữ liệu cấu trúc phải versioned. Sử dụng công cụ migrations (Ví dụ `diesel_migrations`) để đảm bảo schema thay đổi theo thời gian. Mỗi migration được ghi version tăng dần, tự track trong DB. 
- Theo SemVer, tương ứng commit làm thay đổi bảng (**breaking changes**) phải tăng MAJOR, thêm field mới (backward-compatible) tăng MINOR, fix bug schema tăng PATCH.
- Thông tin version của app và schema nên ghi trong metadata, cho phép cập nhật và hỗ trợ rollback nếu cần (ví dụ lưu version app và version schema trong bảng `meta`).

## 4. Luồng dữ liệu và tương tác API

**4.1 Đăng nhập và tải dữ liệu Threads:**  
Người dùng phải đăng nhập OAuth (đến endpoints của Threads hoặc Meta) để cho phép ứng dụng lấy token. Sau đó, module `threads_api` sử dụng token này để gọi API:
- **Lấy danh sách posts:** Gọi `GET /{user-id}/posts` (hoặc tương đương) để lấy các bài đã đăng gần đây. Cần xử lý phân trang nếu có (loại token có hạn mức 60 ngày, refresh định kỳ).
- **Xây dựng cây thread:** Với mỗi post ID, gọi `GET /{media-id}/conversation` để lấy danh sách tất cả replies của post đó dưới dạng phẳng. Sau đó, trong code nhóm replies theo trường `replied_to` để tạo cấu trúc cây nested (post gốc với replies con, replies của replies, v.v). Đảm bảo lưu trữ `replied_to_id` để truy vết vị trí trong cây.
- **Giới hạn API:** Nếu quota sắp hết, hiển thị cảnh báo và tạm dừng/lập lịch tải thêm. Sử dụng endpoint `GET /{threads-user-id}/threads_publishing_limit` để xem hạn ngạch còn lại (khuyến cáo sử dụng, như tài liệu giới thiệu).

**4.2 Lọc và chọn bài:**  
Sau khi dữ liệu đã được lưu cục bộ, luồng lọc tìm kiếm diễn ra trong phần UI và backend:
- **Tìm kiếm**: Người dùng nhập từ khóa (có thể regex, wildcard) trên nội dung text. Hỗ trợ match nhạy và có thể dùng thư viện Tìm kiếm full-text (như SQLite FTS hay Tantivy) để hiệu quả với nhiều dữ liệu.
- **Lọc nâng cao:** Cho phép lọc theo thuộc tính: ngày tháng (khoảng thời gian), tác giả, độ dài text, độ tương tác (số reply, like, share…), sentiment (tích cực/tiêu cực, dự đoán bằng thư viện NLP), nhãn (tags hoặc manually gắn). Xây dựng hệ thống taxonomy: ví dụ nhóm tags như [A: “nội dung liên quan đời sống”, B: “gameplay/anime”, v.v] nếu phù hợp.
- **Phân loại Comment:** Có thể phân biệt comment tích cực/tiêu cực (hỗ trợ NLP), và tách comment chính thức (người viết post) khỏi bình luận của người khác.
- **Mô hình truy vấn:** Triển khai pattern Builder hoặc DSL để compose filter criteria đa chiều. Có thể lưu trữ các truy vấn đươc dùng nhiều (search history) cho tiện.
- **Giao diện lọc:** Hiển thị danh sách kết quả theo thời gian hoặc điểm rank (xem phần ranking), cho phép người dùng đánh dấu chọn (checkbox) các post/comment cần đưa vào project. Hỗ trợ chọn toàn bộ (select all) hoặc theo nhóm.
- **Batch operations:** Sau khi chọn nhiều, cho phép thay đổi chung: ví dụ gắn tags, hoặc xóa khỏi selection. Đảm bảo UX clear cho việc chọn tag ngữ cảnh trong thread tree (nếu chọn parent thì con tự chọn hay không tùy config).

**4.3 Xếp hạng nội dung:**  
Khi có dữ liệu thực (số lượt like, reply count, v.v), module `ranking` tính một điểm số để ưu tiên hiển thị. Ví dụ thuật toán kiểu Reddit: `score = (upvotes - 1) / (time_since_post^x)`, hoặc mức độ hoạt động: nhiều reply, nhiều tương tác → cao điểm. Nếu không có dữ liệu (ví dụ offline), fallback sang heuristic: nội dung có độ dài lớn, chứa từ khóa phổ biến, sentiment tích cực, hoặc random. Quan trọng là **minh bạch**: đưa ra công thức điểm cộng gộp và hiển thị ví dụ (để người dùng hiểu tại sao mỗi comment được rank như vậy). Luồng UI có thể có chế độ “xếp hạng theo” (lựa chọn từ menu: “Thời gian mới nhất”, “Phổ biến cao nhất”, “Phản hồi nhiều nhất”, v.v).

*(Tham khảo: nhiều nền tảng forums đều sử dụng cơ chế xếp hạng kết hợp thời gian và tương tác để ưu tiên comment đắc giá. Ví dụ, Reddit’s ranking factors bao gồm tốc độ upvotes, số comment, thời gian decay…)*

## 5. Màn hình và trải nghiệm người dùng (UX/UI)

**5.1 Luồng người dùng (Creator Workflow):** Mô tả từng bước người sáng tạo tương tác:
1. **Đăng nhập (Login):** Màn hình cấu hình tài khoản Threads (OAuth). Sau khi cấp phép, lưu token offline.
2. **Import Data:** Màn hình chọn tải posts/comments từ Threads. Hiển thị danh sách accounts (nếu hỗ trợ multi-account) hoặc hỏi người dùng nhập ID. Sau khi lấy, hiển thị bảng tóm tắt (số post, số comment).
3. **Review & Lọc:** Giao diện chính với hai phần: **Bảng Posts/Comments** (bên trái) hiển thị danh sách (có thể tree view) và **Panel Lọc** (bên phải) cho phép cấu hình tìm kiếm. Người dùng kiểm tra, sắp xếp, lọc để chọn nội dung. 
4. **Biên tập (Editing):** Sau khi chọn nội dung (đã thêm vào “Project”), chuyển sang màn hình biên tập. Màn hình timeline có thanh thời gian (horizontal), nơi hiển thị track video, track text, track audio. Người dùng có thể kéo-thả đoạn content (ví dụ, một comment có voiceover) lên timeline.
5. **Thêm nguồn (Assets):** UI để quản lý assets: ví dụ tab “Backgrounds” (danh sách video/gameplay, có thể preview), tab “Memes/Stickers” (thư viện ảnh CC0), “Audio/Sounds” (nhạc nền, SFX, voice clips). Có thể kéo một asset từ thư viện vào timeline hoặc layer khác. Mỗi asset kèm metadata (nguồn, license) được hiển thị để đảm bảo hợp pháp.
6. **Chỉnh sửa timeline:** Hỗ trợ chỉnh độ dài clip, canh lề (snap to frame), chuyển cảnh (transition effects). Cho phép xem trước segment (preview playback) ngay trên timeline.
7. **Thêm giọng & caption:** Người dùng có thể click để thêm voice-over: ứng dụng sẽ mở form nhập văn bản, chọn giọng TTS (bảng tùy chọn đã mapping nhiều “speaker” tiếng Việt). Đánh dấu để tạo caption tương ứng. Tự động tạo đoạn SRT và chèn layer text tương ứng. Hỗ trợ điều chỉnh timing thủ công nếu TTS timing lệch.
8. **Âm thanh:** Kéo nhạc nền/hiệu ứng xuống track audio. Đánh dấu nếu cần “ducking”: ví dụ nhạc nền giảm âm lượng tự động khi có voice ở track khác. (Tích hợp tính năng audio ducking tự động sẽ đảm bảo voice rõ ràng).
9. **Xem trước (Preview):** Cho phép xem lại toàn bộ video đã chỉnh trong 1 cửa sổ preview (playback, pause, seek). Kiểm tra âm thanh, chú thích. Có thể capture thành thumbnail.
10. **Render:** Khi hài lòng, chuyển sang hàng đợi render (render queue). Cho phép xây dựng nhiều xuất bản (ví dụ: nhiều bản video với độ phân giải khác nhau). Theo dõi tiến trình (progress bar). Sau khi hoàn thành, xuất file MP4 tới thư mục đích.
11. **Review Output:** Màn hình “History” lưu lại các video đã xuất, kèm metadata (ngày, bản version, commit ID). Cho phép xem lại hoặc xóa. 

**5.2 Hệ thống màn hình (Screen-by-screen):**  

- **Màn hình Dashboard/Project:** Cho phép tạo/danh sách dự án, thông tin chung.  
- **Màn hình Import Data:** Giao diện chọn tài khoản, bắt đầu load dữ liệu từ API. Hiển thị log tải.  
- **Màn hình Content Library:** Bảng/Tree view của Posts & Comments đã tải. Khung filter bên cạnh. Cho phép tích chọn nhiều mục. Nút “Add to Project”.  
- **Màn hình Assets Library:** Danh sách ảnh/video/gif, có filter theo loại (meme, background, SFX). Mỗi asset hiện thumbnail và license, người dùng có thể import thêm file mới. 
- **Màn hình Editor/Timeline:** Khu vực xem timeline đa tracks (video track, text track, audio track). Toolbars: text, voice, transitions, undo/redo. Bên dưới timeline có timeline ruler với tick frame chính xác.  
- **Màn hình Preview:** Cửa sổ xem trước video đã dựng, hỗ trợ zoom timeline, tua nhanh/tua lại.  
- **Màn hình Settings:** Thiết lập chung (như output format, độ phân giải, ngôn ngữ, cài đặt TTS default, branch strategy).  
- **Notifications / Logs:** Thanh status hiển thị tiến trình và thông báo lỗi. Hệ thống log (observability) ghi lỗi ra file log. 

*(Chú ý UI: mọi text hướng dẫn/lỗi phải rõ ràng, có icon/chỉ dẫn màu sắc để tránh hiểu nhầm. Khi dùng TTS, hiển thị ví dụ phát âm mẫu. Cung cấp tooltip giải thích các cấu hình kỹ thuật.)*

**5.3 Tiếp cận người khuyết tật (Accessibility):** Giao diện đáp ứng tiêu chuẩn WCAG 2.1/2.2 (ví dụ độ tương phản cao, scalable text, hỗ trợ bàn phím thao tác, screen reader). Ví dụ, tất cả ảnh biểu diễn nội dung phải có `alt text`, các control UI có label, video preview có caption (SRT) và miêu tả ngắn. Hỗ trợ đa ngôn ngữ: giao diện chính tiếng Việt, có thể mở rộng thêm tiếng Anh thông qua hệ thống localization (Rust crates như `fluent` hoặc `gettext`). Văn bản và nội dung media (memes) nên hỗ trợ thay đổi kích thước/phông chữ cho dễ đọc (WCAG A/AA).

## 6. Lọc chi tiết và mô hình truy vấn

**6.1 Phân loại filter (Filter taxonomy):**  
- **Theo từ khóa:** tìm kiếm full-text trên `text` (supports chứa cụm hoặc regex).  
- **Theo tác giả:** lọc bình luận của một danh sách username/country nhất định (Threads API hỗ trợ geo restriction khi đăng).  
- **Theo loại nội dung:** bài gốc vs comment, chỉ hiển thị post có media (image/video) vs text thuần túy.  
- **Theo thời gian:** phạm vi ngày, hoặc sắp xếp (mới nhất/truy cập mới).  
- **Theo chỉ số tương tác:** số lượt reply, like, định lượng phản hồi (Threads cho biết `hasReplies` hay `reply_count`).  
- **Theo sentiment/loại:** dương/âm, phản hồi / normal, nếu đã đánh dấu (tag) trước đó.  
- **Theo topic/từ khóa tùy chỉnh:** cho phép người dùng gắn tag thủ công cho post/comment (ví dụ “meme”, “highlight”), và lọc theo tag.  
- **Điều kiện logic:** kết hợp AND/OR khi có nhiều filter.

**6.2 Mô hình truy vấn:** Triển khai dùng hệ thống biểu diễn truy vấn (chẳng hạn JSON query hoặc biểu thức Rust). Ví dụ, model dạng: 
```yaml
filter: {
  text_contains: "ai trong",
  authors: ["userA", "userB"],
  date_range: ["2026-01-01", "2026-12-31"],
  min_replies: 5,
  sentiment: "positive"
}
```
Phần mềm sẽ chuyển sang câu lệnh SQL/FTS tương ứng (nếu dùng SQLite, điều chỉnh `WHERE` với `MATCH`), hoặc truyền vào bộ lọc của thư viện search. Đảm bảo mô hình này rõ ràng để mở rộng (có thể load query từ file, hoặc nối vào API). Thực hiện test chi tiết cho từng kiểu filter (bao gồm kết quả chính xác, performance). 

*(Nghiên cứu: Ví dụ Elasticsearch Query DSL cho phép filter phức tạp (bool, nested); Ta có thể học theo idea: tách filter logic, query fulltext riêng.)*

## 7. Xếp hạng và chấm điểm bình luận (Comment Scoring)

**7.1 Số liệu thật (Ground-truth) vs Heuristic:** Nếu ứng dụng thực sự tích hợp với API mạng (Threads), có thể lấy chỉ số như `like_count`, `reply_count`, hay `time_created`. Lập thuật toán xếp hạng **dựa trên dữ liệu này**: ví dụ 
- **Điểm tương tác:** có thể là công thức như `(reply_count + like_count*0.5) / (hours_since_post+2)`.  
- **Ưu tiên tương tác sớm:** mới ra nhiều tương tác thì khả năng được quan tâm cao.  
- **Hạn chế lạm dụng:** vì không phải API nào cũng cho số upvotes, nếu thiếu thì fallback. 

Nếu **không có dữ liệu**, dùng phương pháp heuristic:
- Nội dung dài có khả năng nhiều thông tin => tăng điểm (có hoặc không, tùy context).  
- Xếp nhóm theo từ khóa: nếu chứa cụm quan trọng, tăng ưu tiên.  
- Tính sentiment (dịch vụ AWS Comprehend hay Huggingface) để xếp câu tích cực lên đầu.  

**7.2 Minh bạch điểm số:** Mỗi bình luận nên hiển thị nhãn (label) thể hiện điểm số, và cho phép người dùng hiểu công thức (ví dụ tạo trang help giải thích cách xếp hạng). Hai trường hợp cần tách riêng:
- **Real-score (dữ liệu thật):** Gốc từ API (số like, reply). 
- **Heuristic-score:** bản đoán thêm nếu thiếu info.
Đảm bảo UI cho phép “sort by score” hoặc “filter top comments” dựa trên xếp hạng.

*(Tham khảo nghiên cứu: Thuật toán xếp hạng của Reddit hay Hacker News kết hợp votes và thời gian để ưu tiên nội dung mới nhất nhưng vẫn được quan tâm.)*

## 8. Âm thanh, TTS và Phụ đề

**8.1 Text-to-Speech tiếng Việt:**  
- Sử dụng dịch vụ TTS hỗ trợ tiếng Việt, đa giọng. Ví dụ Google Cloud Text-to-Speech (Gemini), AWS Polly, Azure Text-to-Speech, hoặc Coqui TTS. Ví dụ Google TTS cung cấp nhiều giọng Việt: Neural2 và WaveNet (vi-VN-Neural2-A/D, vi-VN-Wavenet-A-D) cùng giọng Standard.  
- Hỗ trợ **đa-speaker**: cho phép người dùng chọn giữa vài giọng (nam/nữ, vùng miền nếu có). Đảm bảo mapping dễ hiểu (ví dụ “Giọng nữ miền Bắc (vi-VN-Neural2-A)”).  
- Đánh giá chất lượng (benchmark): Nên thử so sánh các dịch vụ: latency, phí, chất lượng âm thanh. Tại thời điểm 2026, Gemini 3.8 Flash TTS của Google hứa hẹn studio-grade . Minh bạch trong tài liệu: ghi rõ phiên bản API và chất lượng thu âm thu được.  
- Lưu ý cấm clone giọng: không sử dụng mô hình tùy chỉnh của người dùng, chỉ dùng voice có sẵn. Bảo mật: API keys cho TTS cũng phải lưu an toàn (GitHub Secrets), không commit vào repo.

**8.2 Quản lý audio:**  
- Các track nhạc nền và SFX: chỉ dùng audio royalty-free hoặc CC. Ví dụ thư viện nhạc miễn phí (như Free Music Archive, thương mại có license CC BY). 
- **Ducking**: khi có voiceover, tự động giảm âm lượng nhạc nền để giọng rõ ràng. Kỹ thuật: sidechain compression (filter `sidechaincompress` của FFmpeg). Ví dụ, khi merge audio: `[music][voice]sidechaincompress=threshold=0.01:ratio=20` để duck nhạc tự động khi phát hiện giọng.  
- Sync âm thanh: ghi nhớ tần số mẫu (Hz) và số kênh để tránh lệch nhịp. Nếu cần, resample audio (FFmpeg `aresample`) cho nhất quán giữa track.  
- Xuất phụ đề: Dữ liệu text cho voiceover tạo file `.srt` (phút:giây:mili cú pháp). Xác thực bằng cách dùng timestamps từ TTS hoặc tính dựa trên tốc độ nói. Không tự tạo thời gian ảo (báo cáo nếu chất lượng timing không chắc). Các tiêu chí: mỗi dòng caption < 2s, tránh text quá dài. Có thể cho phép chỉnh sửa timing thủ công.

**8.3 Âm thanh và SFX:**  
- Hỗ trợ thêm hiệu ứng âm thanh (SFX) từ kho (ví dụ tiếng vỗ tay, tiếng click, tiếng “ding” khi chuyển cảnh). Cung cấp một số mẫu miễn phí hoặc link đến kho CC0.  
- Quản lý hiệu ứng gọn: mỗi SFX gắn với event (như “start”, “end”, “transition”). Giao diện timeline có chèn đơn giản.

## 9. Bản dựng video (Rendering)

**9.1 Engine rendering:**  
- Sử dụng ffmpeg (qua binding Rust) hoặc GStreamer để xuất MP4. Với ffmpeg có thể sử dụng filter graph kết hợp video và audio. Ví dụ, lệnh: 
  ```
  ffmpeg -i video1.mp4 -i video2.mp4 -filter_complex "[0:v][0:a][1:v][1:a] concat=n=2:v=1:a=1[outv][outa]" -map "[outv]" -map "[outa]" output.mp4
  ``` 
  (Thực ra timeline có thể phức tạp với nhiều layer, nhưng nguyên tắc ghép nối tương tự.)  
- Đảm bảo **frame-accurate:** mọi đối tượng trên timeline phải xuất đúng frame hiển thị. Sử dụng dấu thời gian (timestamps) tuyệt đối khi ghép audio. Kiểm thử đầu ra bằng so sánh checksum frame với kỳ vọng (images).
- Đầu ra MP4: chuẩn mã hóa H.264 (or H.265) trong container MP4, âm thanh AAC. Cho phép chỉnh cấu hình (bitrate, resolution) theo ý người dùng.  
- Trong Dev-phase, kiểm thử performance bằng thử xuất file dài (ví dụ 5 phút) trên máy tính thấp tài nguyên. Tài liệu cho biết ffmpeg có thể tận dụng đa luồng CPU để tăng tốc. Đảm bảo giới hạn: nếu quá chậm, khuyến khích dùng GPU encode (nvidia/amd) nếu khả dụng.  
- **Tính deterministic:** Mặc dù ffmpeg có thể cho ra bitstream khác giữa lần chạy (không ảnh hưởng nội dung visual), nhưng khuyến khích lưu thông tin seed (nếu dùng bộ lọc ngẫu nhiên). Với timeline đã xác định, pixel phải giống nhau bất kể thiết bị. Nếu API cần random (ví dụ hiệu ứng ngẫu nhiên), yêu cầu lưu lại seed hoặc fix trước khi render.

**9.2 Tích hợp preview:**  
- Cho phép preview từng phân đoạn ngay trong UI: ví dụ chạy tạm timeline lên texture, không cần render full file. Có thể dùng thư viện multimedia (SDL2, FFplay) để chơi nhanh.  
- Tính năng này không cần bitstream file, nhưng vẫn đảm bảo khung hình hiển thị đúng.  
- Phát hiện lỗi: nếu video bị giật khung, ghi log, hiển thị user cảnh báo.

## 10. Lưu trữ, bộ nhớ đệm và phục hồi

**10.1 Lưu trữ:**  
- Dữ liệu lớn (video, audio) không nên lưu trực tiếp trong DB. Chỉ lưu **path** đến file nằm ở thư mục dự án của ứng dụng. Đảm bảo folder cấu trúc rõ ràng (ví dụ `projectX/assets/`).  
- Files được copy/sao chép vào thư mục project để tránh mất khi source gốc thay đổi.  
- Tính năng backup: cho phép người dùng xuất bản sao dự án (project export) với toàn bộ assets và DB (ví dụ zip hoặc copy) để có thể phục hồi trên máy khác.  
- Thư mục cache: Tạm lưu các file tạm khi render. Đảm bảo xóa sạch sau khi xuất.

**10.2 Phục hồi sau lỗi:**  
- Sử dụng transaction (SQLite) hoặc error handling đảm bảo DB không bị corrupt khi crash.  
- Khi khởi động lại ứng dụng, nếu phát hiện crash giữa chừng (ví dụ file incomplete), xác định tình trạng: nếu đang render dở, cho tùy chọn resume hoặc hủy.  
- Log và stack trace: Tích hợp thư viện logging (ví dụ `slog`), ghi file log lỗi chi tiết (với timestamp) để troubleshooting. Bao gồm cả logs build từ CI nếu cần.

**10.3 Caching:**  
- Caching trên CI: Sử dụng cache action (GitHub Actions) để lưu trữ `.cargo/registry` và thư mục target.
- Caching trong ứng dụng: Ví dụ caching kết quả API (trên 24h) để giảm request. Lưu kết quả Threeds API theo token để tái sử dụng, với TTL phù hợp.

## 11. Xử lý lỗi và quan sát (Observability)

- **Error Handling:** Mọi API call/network call phải bọc error, hiển thị lỗi thân thiện (thay vì panic). Ví dụ, nếu Threads trả 401 do token hết hạn, hiện thông báo “Token đã hết hạn, vui lòng đăng nhập lại”.  
- **Logs:** Ghi log chi tiết với mức độ (INFO/DEBUG/ERROR). Định nghĩa log format (JSON hoặc plain) kèm timestamp.  
- **Monitoring:** Cung cấp console log xem tiến độ long-running tasks (ví dụ render) và hoàn thành. Dự định dùng “Loki or Prometheus” cho service backend (nếu mở rộng microservices) – trong desktop app, chủ yếu UI logger.  
- **Debugging:** Trong dev mode, hiển thị thêm thông tin stacktrace khi lỗi hoặc capture dump logs để ghi lại bug.

## 12. Mô hình đe dọa & tuân thủ

- **Quyền truy cập:** Không cố vượt qua hạn chế API của Threads (ví dụ không brute-force lấy hết comments của user khác). Chỉ tác động lên nội dung user đã login. 
- **Bảo vệ secret:** Không commit API keys, tokens. Các key (Threads app secret, TTS API key) phải lưu trong GitHub Secrets hoặc file config ngoại vi (user config). Trong mã nguồn, không hardcode.
- **Bản quyền nội dung:** Chỉ cho phép import asset “miễn phí” (CC0/CC-BY hoặc public domain). Giao diện phải hiển thị rõ license của mỗi meme/nhạc. Nghiên cứu dán nhãn: ví dụ, tham khảo Wikipedia “Public domain” hoặc Creative Commons để hướng dẫn người dùng sử dụng đúng luật. (CC0 cho phép dùng tự do mà không cần ghi credit.)
- **Giọng nói:** Không clone giọng thật nếu không có consent. Nếu người dùng cung cấp audio riêng, phải cảnh báo quyền riêng tư. TTS dùng giọng có sẵn hợp đồng với nhà cung cấp.
- **GDPR/Privacy:** Nếu lưu trữ dữ liệu cá nhân (usernames, avatar), xác định rõ trên config file của ứng dụng (local-first có nghĩa data không rời khỏi máy người dùng). Không có server trung gian thu thập.
- **Threat model:** Đánh giá ít nhất 1. SQL injection (nếu dùng SQLite, tránh concatenation, dùng prepared statements), 2. Lỗ hổng XSS/CSRF (không áp dụng vì desktop app native), 3. Rò rỉ thông tin (tránh log token hoặc email). Document nêu ra biện pháp phòng tránh.
- **Giấy phép:** Mã nguồn nên được license rõ (ví dụ Apache-2.0 hoặc MIT) tuân thủ open-source. Tài liệu thuộc (images, âm thanh) phải tuân giấy phép liên quan (ví dụ CC BY thì ghi credit).

## 13. Accessibility (Khả năng tiếp cận)

- **WCAG compliance:** Giao diện phải tương thích WCAG 2.1 Level AA. Ví dụ: dùng màu có độ tương phản ≥4.5:1 cho text, hỗ trợ tăng kích thước chữ, cấu trúc heading đúng thứ tự. Các control (nút, link) phải có label rõ cho screen reader.  
- **Keyboard-first:** Tất cả tính năng phải có phím tắt, hỗ trợ tab navigation đầy đủ (Ví dụ: phím Space/Enter chọn, Esc đóng dialog).  
- **Localization:** Nội dung có thể chuyển đổi giữa tiếng Việt và tiếng Anh (hoặc thứ ngôn ngữ khác) dễ dàng bằng cách sử dụng file ngôn ngữ (JSON, YAML). Font Unicode an toàn cho tiếng Việt (có dấu).  
- **Chú thích video:** Phụ đề (captions) cho tất cả video xuất ra (dựa trên nội dung voice hoặc văn bản). Đảm bảo định dạng tiêu chuẩn (.srt) để media player/YouTube có thể hiển thị đúng.  
- **UI readability:** Tránh dùng màu “nổi” gây lóa, thay vào đó dùng theme nhất quán. Các nút quan trọng (Play, Export) dùng icon kèm label.

*(Trích dẫn: The World Wide Web Consortium (W3C) khuyến cáo tuân WCAG để nội dung dễ tiếp cận với người khiếm thị, khiếm thính, mất khả năng vận động….)*

## 14. Lộ trình và tiêu chí nghiệm thu (Acceptance Criteria)

**14.1 Lộ trình chức năng:** (Ưu tiên MVP trước, sau đó thêm tính năng nâng cao)
- **MVP (Phase 1):** Đăng nhập Threads, tải posts + comment, hiển thị danh sách, filter cơ bản, tạo timeline với text overlay, xuất video MP4 (không cần voice). CI/CD cơ bản hoạt động.  
- **Phase 2:** Thêm TTS tiếng Việt, caption đồng bộ, nhập thêm assets (hình ảnh meme, âm thanh). Audio ducking.  
- **Phase 3:** Xếp hạng comment nâng cao, UI refinement (transitions, thiết lập giọng, presets). Demo một ví dụ video có voice/âm thanh hoàn chỉnh.  
- **Phase 4:** Tích hợp đa nền tảng (Windows/Mac/Linux), đảm bảo performance. Ra mắt bản public (v3.0.0), cùng tài liệu hướng dẫn.  

**14.2 Ma trận nghiệm thu (Acceptance Matrix):**  

| Tính năng/Có thể | Tiêu chí nghiệm thu (Có/Không)        | Chỉ số đo đếm                      |
|------------------|--------------------------------------|------------------------------------|
| **Data Import**  | - Lấy thành công N bài có cấu trúc nhánh replies.<br> - Không bắn ngoại lệ khi token hết hạn (cảnh báo, yêu cầu refresh). | Hiệu quả tải, độ ổn định API (<5% lỗi). Thời gian tải <30s/100 post. |
| **Filtering**    | - Tìm ra đúng post/comment theo từ khóa.<br> - Lọc theo tag/user/date/...đúng. | Soát lỗi: >90% query test chính xác. |
| **Ranking**      | - Thuật toán không sắp xếp sai thứ tự quan trọng (minh chứng qua ví dụ thử nghiệm thực tế).<br> - Thể hiện rõ “score” hay “hotness”. | Độ chính xác logic (unit test); mô tả công thức rõ ràng. |
| **Timeline Editor** | - Kéo-thả được assets lên timeline, chỉnh độ dài frame.<br> - Undo/redo hoạt động.<br> - Clip tạm preview đúng. | Functionality: ✓ video được preview đúng; ✓ layer xử lý đúng lệnh. |
| **TTS & Audio**  | - Voice ra tiếng Việt chuẩn, mapping đúng (speaker nam/nữ).<br> - Phụ đề (.srt) đồng bộ với audio.<br> - Nhạc nền duck đúng khi voice play. | Độ chính xác giọng, latency TTS: <500ms phê duyệt. Audio duck: volume giảm 10-15dB khi voice. |
| **Rendering**    | - File MP4 xuất ra xem được, frame continuity, không crash.\n- Chính xác màn final vs timeline mẫu (deterministic). | So sánh CRC/frame; thời gian render (benchmark máy) trong giới hạn. |
| **UI/UX**        | - Màn hình sạch, vận hành mượt (<60fps preview).<br> - Bảng điều khiển dễ hiểu (test với người dùng). | Độ hài lòng (user test), speed (<100ms phản hồi filter). |
| **Accessibility**| - Có thể điều khiển toàn bộ bằng bàn phím.<br> - Tất cả text có độ tương phản hợp WCAG. | Kiểm thử WCAG (dùng công cụ); đủ nhãn alt/text cho images. |
| **CI/CD & Quy trình** | - Code build & test tự động trên PR với GitHub Actions (không có lỗi).<br> - Tất cả commit theo Conventional Commits.<br> - Branch bảo vệ yêu cầu review và status checks. | PR passes CI, code coverage đạt threshold, có docs hướng dẫn devops. |
| **GitHub Workflow** | - Có GitHub Actions YAML mẫu thực thi được (lint, test, build).<br> - Automatic release (tag), SBOM (nếu khả thi). | Workflow run success 100% trên push/master và PR. |

**Definition of Done (DoD):** Mỗi user story/issue xem là hoàn thành khi:
- Code tương ứng đã được commit, reviewed, merge vào nhánh main theo quy chuẩn (commit msg hợp lệ, unit test đi kèm).
- Tất cả unit/integration tests liên quan đã pass trong CI (log xác nhận).
- Tài liệu tương ứng (cập nhật spec, ADR, README, CHANGELOG) đã hoàn thành và được duyệt.
- Bản build mẫu (artifact, demo video) được tạo ra và kiểm chứng. Ví dụ, nếu thêm chức năng voice, cần đính kèm 1 short video sample để QA review.
- Mức độ code coverage không giảm dưới mức đã set (ví dụ 80%).
- Issue tracker ghi rõ commit ID hoặc PR đã được merge.
- Không có lỗi nghiêm trọng (crash) hoặc security warnings trước khi release.

## 15. Quy trình GitHub & CI/CD

**15.1 Chiến lược nhánh (Branch Strategy):** Theo **Gitflow** hoặc **Trunk-based**:
- Nhánh chính `main` (release-ready), `develop` (nếu dùng) hoặc trực tiếp dùng `main` với feature branches. Tất cả merge vào main phải qua Pull Request được review (ít nhất 1 reviewer). Đề xuất dùng **branch protection rules** buộc review và status check trước merge.
- Feature branches: đặt tên theo thể loại (ví dụ `feat/filter-search`, `fix/audio-timing`). Kết hợp **Conventional Commits**: commit dạng `type(scope): message`, ví dụ `feat(ranking): add scoring function for comments`. Dùng chuẩn này giúp tự động ghi CHANGELOG và versioning.
- Không dùng force-push trên main hoặc feature đã public. Nếu cần sửa lịch sử, làm revert commit thông thường.

**15.2 Commit và versioning:**  
- **Conventional Commits:** Mọi commit phải tuân dạng `type(scope): short description`. Các loại `fix`, `feat`, `chore`, `docs`, `refactor`, `test`, `ci` v.v. Nếu commit gây **breaking change**, thêm đoạn `BREAKING CHANGE:` trong nội dung hoặc `!` ngay sau type (vd `feat!: change API`). Sử dụng Type này để tự động xác định tăng version (fix → patch, feat → minor, breaking → major).
- **Phiên bản:** Áp dụng **Semantic Versioning** cho cả release của ứng dụng. Dùng tag git form `vX.Y.Z`. Mọi release từ CI (hoặc thủ công) phải tạo tag kèm ghi CHANGELOG tương ứng.
- **CHANGELOG:** Tự động tổng hợp từ commit messages theo Conventional Commits (có công cụ như `conventional-changelog` hoặc GitHub action). Nêu rõ các tính năng, sửa lỗi, BREAKING CHANGES.  

**15.3 Pull Request (PR) Workflow:**  
- Mỗi tính năng/bug fix tạo branch riêng, khi xong thì mở PR. PR nên mô tả ngắn gọn (tiếng Việt nhưng có thuật ngữ đặc thù là tiếng Anh nếu cần), liệt kê các chức năng thay đổi. Kèm link đến issue nếu có. 
- CI tự kích hoạt: chạy format checker (Rustfmt), linter (Clippy), type-check, unit tests. Chỉ cho merge nếu mọi bước CI pass (fail một step -> PR blocked).
- Yêu cầu review bởi ít nhất 1 thành viên khác. Mọi thay đổi chỉ được merge khi có phê duyệt. Review chú ý coding style Rust, logic phù hợp spec, an ninh.  
- Sau merge, CI chạy release build: build production binary, chạy test e2e (nếu có), upload artifact. Đảm bảo tagged version tương ứng (như plugin GitHub action create-release).

**15.4 GitHub Actions CI:**
- **Workflow mẫu (sample):**  
  ```yaml
  name: CI

  on:
    push:
      branches: [main, develop]
    pull_request:
      branches: [main]

  jobs:
    lint-format:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v6
        - name: Rustfmt
          run: cargo fmt -- --check
        - name: Clippy
          run: cargo clippy --all -- -D warnings

    build-test:
      needs: lint-format
      runs-on: ${{ matrix.os }}
      strategy:
        matrix:
          os: [ubuntu-latest, windows-latest, macos-latest]
      steps:
        - uses: actions/checkout@v6
        - name: Cache dependencies
          uses: actions/cache@v4
          with:
            path: |
              ~/.cargo/registry
              ~/.cargo/git
              target
            key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }} 
        - name: Setup Rust
          uses: actions-rs/toolchain@v1
          with:
            profile: minimal
            toolchain: stable
        - name: Build (release)
          run: cargo build --release
        - name: Run tests
          run: cargo test --all -- --nocapture

    e2e:
      needs: build-test
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v6
        - name: Run E2E (sample)
          run: |
            # Chạy ứng dụng với sample data; ghi log
            ./target/release/antares --run-e2e-tests
        - name: Save CI Artifacts
          uses: actions/upload-artifact@v4
          with:
            name: antares-ci-artifact
            path: ./target/release/antares
  ```
  Workflow trên cho phép **đa nền tảng** (Ubuntu, Windows, MacOS) với matrix, bao gồm:
  - `formatting` và `linter` (Clippy) gộp trong job đầu.
  - `build-test` job có `cache` (dựa trên `Cargo.lock`). Sử dụng `actions-rs/toolchain` để cài Rust.
  - `e2e` job minh họa: chạy kịch bản tích hợp, có thể thực hiện render video mẫu và so khớp với output mong đợi (QA artifact).
  - Tích hợp **Upload artifact** giúp thu thập file nhị phân, logs; dùng cho release tiếp theo.
- **Kiểm tra bổ sung:** 
  - Nếu dùng TypeScript/CSS trong UI, thêm job chạy ESLint/Prettier/TSC theo tương tự.
  - Có thể thêm job “security scan” dùng cargo-geiger hoặc OSS scanners, nếu quan tâm SBOM.
  - Đảm bảo CI có quyền sử dụng GitHub Token cho thao tác đóng PR, tạo release nếu có script.

**15.5 Cổng kiểm thử (Test gates) và bảo vệ nhánh:**  
- Kích hoạt **branch protection** trên `main`: yêu cầu ít nhất 1 review và trạng thái CI passing để merge.
- Không cho phép merge nếu có conflict.  
- Đặt rule bảo vệ secret: với `GITHUB_TOKEN`, các job deploy/release tự động không lộ secret. Dùng `actions/upload-artifact` để lưu thông tin thay vì echo secrets.
- **Rollback:** Mỗi release tag chứa bản build có thể rollback. Trong trường hợp phát hiện bug nghiêm trọng, quy trình rollback: revert commit tương ứng, tăng version patch hoặc hotfix branch. Không dùng force-push trên main. Có thể hỗ trợ script automated `gh release delete` nếu cần.

**15.6 Quản lý phụ thuộc:**  
- Khóa phiên bản Cargo (`Cargo.lock`) đảm bảo xác định. Dùng `actions/cache` cho `~/.cargo` đã nêu.  
- Cập nhật library: Định kỳ kiểm tra bản mới nhất của thư viện (do Rust stable cầu tiến nhanh). Xem xét vulnerabilites (RustSec advisories).
- **SBOM/signing:** Trong phạm vi có thể, tạo **bill of materials** (dùng `cargo-about` hoặc `syft`) và đính vào release notes. Ký số binary (ví dụ GPG, OpenPGP) nếu khả thi.  
- **Secret management:** Sử dụng GitHub Secrets cho keys (Threads, TTS, crates.io). Không commit config bí mật. Document chỉ cách thiết lập secrets trong CI (ví dụ `GH_TOKEN`, `TTS_API_KEY` trong Secrets UI).

**15.7 Versioning & Release Workflow:**  
- Sau mỗi milestone hoàn thành, dùng tag `vX.Y.Z` tạo Release trên GitHub (có thể tự động bằng GitHub Action). Ghi chú release (release notes) từ CHANGELOG tương ứng.  
- Dùng semantic-release tool hoặc workflow tự động tăng version dựa trên commits.  
- Tham khảo spec SemVer: tăng major/minor/patch đúng quy tắc.  
- Bắt buộc phải có **Git tag** trước khi build release; nếu build thất bại, không lưu tag (thuật ngữ: “Commit-by-commit verification”).

**15.8 Tài liệu quy trình Agent:**  
- Quy định rõ agent (nếu có dùng LLM bots) chỉ commit/PR khi được phép (ví dụ comment Slack “Agent: Proposed changes”).  
- Mỗi commit của agent phải kèm log (artifact) chứng minh đã chạy thành công test và render (ví dụ attach screenshot video, logs). Agent không được bỏ qua test failures hoặc thực hiện force push.  
- Cấu hình workflows sao cho mỗi PR của agent sẽ chạy full test, và có reviewer người thật phê duyệt trước khi merge/release.

## 16. Thay đổi (Changelog) từ V2 → V3

- **Bổ sung chi tiết**: Dữ liệu/schema với versioning và migrations; các mô-đun riêng biệt (`threads_api`, `filter`, `ranking`, `renderer`, v.v). 
- **Mở rộng UX**: Luồng người dùng cụ thể, màn hình từng bước; timeline UI; assets library; hiệu ứng chuyển cảnh; accessibility; localization.  
- **Lọc nâng cao**: Phân loại filter chi tiết, query model (V2 chỉ liệt kê sơ qua).  
- **Xếp hạng bình luận**: Tách rõ thuật toán dựa trên dữ liệu và heuristic. 
- **Tích hợp TTS và âm thanh**: Đề cập công nghệ TTS Tiếng Việt hiện đại, audio ducking, SFX. (V2 chỉ nhắc lồng tiếng, chưa có benchmark hay multi-speaker chi tiết).
- **Hiện thực render**: Thêm yêu cầu frame-accuracy, perf, deterministic. 
- **Lưu trữ/Cache/Recovery**: Bổ sung hạ tầng dữ liệu (DB, backup, transaction).
- **Accessibility/Localization**: Bổ sung tính năng hỗ trợ người khuyết tật và đa ngôn ngữ (V2 thiếu).  
- **Xử lý lỗi/Observability**: Chi tiết log, monitor, debug. (V2 chỉ nói qua).  
- **Security/Compliance**: Nêu rõ threat model, GDPR, license compliance (V2 có but chưa chi tiết).  
- **Roadmap & Acceptance**: Thêm phân đoạn lộ trình và bảng kiểm nghiệm cụ thể.  
- **GitHub Automation**: Mở rộng rất lớn: commit conventions, branch protection, workflow, caching, release notes, SBOM, v.v. V2 chỉ có đề cập sơ lược.  
- **ADR**: Bổ sung yêu cầu ghi lại ADR để theo dõi quyết định thiết kế (V2 không có).
- **Các quyết định chưa xác minh**: V3 giả định tích hợp Google/Azure TTS, ffmpeg rendering, nhưng cần kiểm thử thực tế. (Danh sách kiểm: “có thể dùng Google Gemini TTS”, “ffmpeg binding cho Rust”).  
- Mọi **guardrail** (không scrap, local-first, privacy, no secrets) vẫn duy trì như cũ.  

## 17. Bảng tham khảo (References)

- Threads API documentation và giới hạn (Outstand blog tham khảo tài liệu Meta).  
- Quy chuẩn **Conventional Commits** (commit message format).  
- **GitHub Actions** (Rust CI/CD example, caching).  
- **Branch protection** (GitHub docs).  
- **Semantic Versioning** (quy tắc versioning).  
- **Google Cloud TTS** (danh sách giọng tiếng Việt).  
- **Audio ducking/Sidechain** (FFmpeg guide).  
- **Architecture Decision Records** (Martin Fowler).  

Mỗi tài liệu tham khảo đã được truy xuất (accessed 2026) để đảm bảo thông tin cập nhật. Các phần không có trích dẫn rõ là dựa trên nguyên tắc chung/kinh nghiệm kỹ thuật (documentation). 

