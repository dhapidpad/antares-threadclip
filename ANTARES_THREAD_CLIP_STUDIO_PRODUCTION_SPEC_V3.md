# ANTARES_THREAD_CLIP_STUDIO_PRODUCTION_SPEC_V3.md

> **Antares ThreadClip Studio** – Desktop Creator Studio *local-first* biến post/comment Threads (của chính người dùng) thành video short-form MP4 chính xác từng khung hình.
> Tài liệu này là **đặc tả chuẩn để coding agent thực thi và tự kiểm chứng**. Mọi yêu cầu dùng từ khóa **PHẢI / KHÔNG ĐƯỢC / NÊN** (MUST / MUST NOT / SHOULD).

| Trường | Giá trị |
|---|---|
| Phiên bản spec | 3.0.0 (nâng cấp từ V2) |
| Trạng thái | Draft-for-implementation |
| Ngôn ngữ UI mặc định | Tiếng Việt (có i18n sang English) |
| Nền tảng đích | Windows 10/11 (ưu tiên), macOS, Linux |
| Stack mặc định | Rust (core) + Tauri 2 + React/TypeScript/Vite (UI) + SQLite + FFmpeg |
| Quyền agent mặc định | **L1 – chỉ làm local**, xem mục 17.3 |

---

## 0. Cách dùng tài liệu này (dành cho agent)

1. Đọc **mục 1 (Guardrails)** và **mục 17 (Giao thức agent)** trước khi viết bất kỳ dòng code nào.
2. Làm việc theo **Milestone → Task** ở mục 18. Mỗi task có ID (`M1-T03`), tiêu chí nghiệm thu dạng Given/When/Then và *bằng chứng* phải nộp.
3. Mục **2 (Sổ xác minh)** liệt kê các giả định chưa được kiểm chứng. **KHÔNG ĐƯỢC** coi chúng là sự thật; phải chạy *spike* (thử nghiệm nhỏ), ghi kết quả vào `docs/verification/`.
4. Quyết định kiến trúc đi vào ADR (`docs/adr/`). **KHÔNG ĐƯỢC** sửa ADR đã `Accepted`; tạo ADR mới thay thế.
5. Không báo "hoàn thành" nếu thiếu bằng chứng (log test, artifact render, ảnh chụp). Báo trung thực: `PASS / FAIL / BLOCKED / UNVERIFIED`.

---

## 1. Mục tiêu, phi mục tiêu, guardrails

### 1.1 Mục tiêu
- **G1 Import:** đăng nhập OAuth, tải post + replies của *chính tài khoản đã ủy quyền*, lưu cục bộ, dựng được cây thread.
- **G2 Chọn lọc:** tìm kiếm full-text, lọc đa chiều, xếp hạng minh bạch, chọn hàng loạt.
- **G3 Dựng video:** timeline đa track (nền, thẻ chữ, meme, audio), preview, undo/redo, snap theo frame.
- **G4 Âm thanh:** TTS tiếng Việt đa giọng, caption SRT/WebVTT, ducking, mix nhiều track.
- **G5 Render:** MP4 H.264 + AAC, **frame-accurate**, deterministic trên cùng input.
- **G6 Quy trình:** GitHub automation an toàn (CI đa nền tảng, Conventional Commits, SemVer, release có điều kiện).

### 1.2 Phi mục tiêu (Non-goals)
- KHÔNG crawl/scrape nội dung người khác hoặc vượt giới hạn API.
- KHÔNG đăng bài/sửa bài lên Threads (V3 chỉ **đọc**; xem 5.5).
- KHÔNG có server trung gian, KHÔNG telemetry gửi ra ngoài mặc định.
- KHÔNG clone giọng người thật.
- KHÔNG đưa ứng dụng lên cloud hay làm đa người dùng.

### 1.3 Guardrails (bất biến – vi phạm = FAIL)
| ID | Quy tắc |
|---|---|
| GR-01 | **Local-first:** dữ liệu gốc lưu trên máy; chỉ gọi API đã được ủy quyền. |
| GR-02 | **Privacy-first:** không log token/email/nội dung riêng tư; không gửi dữ liệu ra ngoài khi chưa có opt-in. |
| GR-03 | **Không secrets trong repo**; secrets chỉ ở OS keychain (runtime) hoặc GitHub Secrets (CI). |
| GR-04 | **Không clone giọng** khi không có consent; chỉ dùng giọng có sẵn của nhà cung cấp. |
| GR-05 | **License-aware:** mọi asset có `license` + `source`; chỉ CC0 / CC-BY / public domain / do người dùng sở hữu. |
| GR-06 | **Không bịa số liệu:** không tạo like/view/metrics giả khi render; caption timing không đoán mò mà không gắn cờ. |
| GR-07 | **Không force-push, không bỏ qua test lỗi, không tuyên bố thành công thiếu bằng chứng.** |
| GR-08 | **Mọi tác động ra ngoài máy (push/PR/release) cần quyền rõ ràng** theo mục 17.3. |

---

## 2. Sổ xác minh (Verification Register)

Báo cáo nghiên cứu gốc chứa nhiều khẳng định chưa kiểm chứng. Bảng dưới là các mục **phải spike trước khi dựa vào**. Kết quả ghi `PASS/FAIL/PARTIAL` + ngày + nguồn vào `docs/verification/V-XX.md`.

| ID | Giả định cần kiểm chứng | Spike | Phương án nếu FAIL |
|---|---|---|---|
| V-01 | Threads API trả replies của post qua `GET /{media-id}/conversation` (và `/replies`) kèm `replied_to`. | Gọi thật bằng tài khoản test. | Chỉ lấy `/replies` từng cấp, dựng cây theo cấp. |
| V-02 | Có thể lấy `like_count`, `reply_count` cho post/reply của mình (thường qua `/insights`, không phải field cơ bản). | Liệt kê field/insight thực tế. | Ranking chuyển sang **heuristic-only** (mục 7.3). |
| V-03 | Hạn mức đọc/ghi thực tế của Threads (số liệu "250 post/ngày, 1000 reply/ngày, 4.800 call/ngày" trong báo cáo gốc). | Đọc tài liệu Meta hiện hành + đo header phản hồi. | Dùng rate-limiter cấu hình được, mặc định thận trọng. |
| V-04 | `threads_publishing_limit` chỉ đo hạn mức **đăng**, KHÔNG phải hạn mức **đọc**. | Đọc docs + gọi thử. | Tự đếm request cục bộ (token bucket). |
| V-05 | Token long-lived cần refresh trước 60 ngày. | Đọc docs; mô phỏng hết hạn. | Thông báo + buộc đăng nhập lại. |
| V-06 | OAuth dùng được cho ứng dụng desktop (redirect `http://127.0.0.1:<port>` / custom scheme, PKCE nếu hỗ trợ). | Dựng thử luồng loopback. | Cung cấp luồng **nhập JSON export thủ công** (5.4). |
| V-07 | Google Cloud TTS có giọng `vi-VN-*` (Standard/WaveNet/Neural2) và trả được timepoint/SSML mark để caption. | Gọi thử, đo latency & timing. | Caption dựa trên forced alignment cục bộ hoặc gắn cờ `timing_estimated`. |
| V-08 | Một TTS **offline** tiếng Việt chất lượng chấp nhận được (Piper/Coqui/…); license phù hợp. | Benchmark mục 10.3. | Cho phép chạy không TTS (chỉ text + nhạc). |
| V-09 | FFmpeg (binary sidecar, không phải binding) đáp ứng frame-accurate + ducking + burn-in caption. | Render bộ mẫu golden (mục 11.4). | Đổi sang pipeline raster-từng-frame → pipe vào FFmpeg. |
| V-10 | "Gemini 3.8 Flash TTS" (nêu trong báo cáo gốc) có tồn tại/khả dụng và giá. | Kiểm tra tài liệu nhà cung cấp. | Bỏ khỏi phạm vi; dùng provider đã xác minh. |
| V-11 | Tauri 2 + WebView đủ cho timeline canvas 60 fps trên máy yếu. | Prototype timeline 200 clip. | Chuyển canvas sang WebGL/Pixi hoặc native (egui). |

> Quy tắc: nếu spike chưa chạy, trạng thái là `UNVERIFIED` và tính năng phụ thuộc phải có **fallback** hoạt động được.

---

## 3. Kiến trúc

### 3.1 Cấu trúc repo (monorepo)

```text
antares-threadclip/
├─ Cargo.toml                  # workspace
├─ crates/
│  ├─ tc-core/                 # domain types, errors, ids, time (frame math)
│  ├─ tc-threads/              # OAuth + Threads REST client + rate limiter
│  ├─ tc-db/                   # SQLite, migrations, repositories
│  ├─ tc-filter/               # query model + compiler -> SQL/FTS5
│  ├─ tc-ranking/              # real-score & heuristic-score
│  ├─ tc-timeline/             # project model, edit ops, undo/redo, validation
│  ├─ tc-audio/                # TTS providers, caption, loudness, ducking graph
│  ├─ tc-assets/               # asset library, license metadata, import
│  ├─ tc-render/               # render plan -> FFmpeg, determinism, golden tests
│  └─ tc-app/                  # Tauri commands (IPC), wiring, config, logging
├─ ui/                         # React + TS + Vite (screens, timeline canvas)
├─ assets/builtin/             # asset gốc kèm LICENSES.md (CC0 / tự tạo)
├─ testdata/                   # fixtures: threads JSON, golden video/frames
├─ docs/
│  ├─ adr/                     # 0001-*.md ...
│  ├─ verification/            # V-01.md ...
│  ├─ spec/                    # file này
│  └─ qa/                      # báo cáo QA do agent sinh
├─ .github/
│  ├─ workflows/               # ci.yml, release.yml, codeql.yml
│  ├─ PULL_REQUEST_TEMPLATE.md
│  └─ CODEOWNERS
├─ CHANGELOG.md
└─ AGENTS.md                   # tóm tắt giao thức agent (trỏ về mục 17)
```

### 3.2 Ranh giới module (quy tắc phụ thuộc)

```text
tc-core  ←  (tất cả)
tc-db ← tc-threads, tc-filter, tc-ranking, tc-timeline, tc-assets
tc-timeline ← tc-render, tc-audio
tc-app  → gọi tất cả; UI chỉ nói chuyện với tc-app qua IPC
```
- `tc-render` **KHÔNG ĐƯỢC** phụ thuộc `tc-threads`/UI. `tc-threads` **KHÔNG ĐƯỢC** biết về timeline.
- Mọi I/O mạng nằm trong `tc-threads` và provider TTS của `tc-audio` (có trait để mock).
- Không `unwrap()`/`panic!` ở đường chạy thường; dùng `thiserror` + `Result`.

### 3.3 ADR bắt buộc (tạo ở M0)

| ADR | Quyết định (mặc định đề xuất) |
|---|---|
| 0001 | Dùng Tauri 2 + Rust core + React/TS UI (thay thế được qua ADR mới sau V-11). |
| 0002 | SQLite (WAL) + `sqlx` hoặc `rusqlite` + migration tuần tự; FTS5 cho tìm kiếm. |
| 0003 | FFmpeg chạy **sidecar process**, không link tĩnh (tránh rủi ro license/GPL; xem 14.4). |
| 0004 | Thời gian timeline tính bằng **frame số nguyên** + `fps` hữu tỉ (không dùng float giây). |
| 0005 | Secrets lưu trong OS keychain (`keyring`), không file text. |
| 0006 | Định dạng project: thư mục + `project.json` (versioned) + `media/`. |

ADR template (≤ 2 trang): `Status · Context · Decision · Consequences · Alternatives`.

---

## 4. Mô hình dữ liệu

### 4.1 Schema SQLite (v1)

```sql
-- meta: version app & schema
CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
-- seed: schema_version, app_version, created_at

CREATE TABLE account (
  id TEXT PRIMARY KEY,             -- threads user id
  username TEXT NOT NULL,
  token_expires_at INTEGER,        -- epoch s; token KHÔNG nằm trong DB
  last_import_at INTEGER
);

CREATE TABLE post (
  id TEXT PRIMARY KEY,
  account_id TEXT NOT NULL REFERENCES account(id),
  text TEXT NOT NULL DEFAULT '',
  author TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  permalink TEXT,
  media_type TEXT,                 -- TEXT_POST | IMAGE | VIDEO | CAROUSEL
  reply_count INTEGER,             -- NULL = không có dữ liệu thật
  like_count INTEGER,              -- NULL = không có dữ liệu thật
  raw_json TEXT                    -- bản gốc để debug/migrate
);

CREATE TABLE reply (
  id TEXT PRIMARY KEY,
  post_id TEXT NOT NULL REFERENCES post(id) ON DELETE CASCADE,
  replied_to_id TEXT,              -- id cha (post hoặc reply)
  depth INTEGER NOT NULL,          -- tính khi dựng cây
  text TEXT NOT NULL DEFAULT '',
  author TEXT NOT NULL,
  is_author_reply INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  reply_count INTEGER, like_count INTEGER,
  raw_json TEXT
);
CREATE INDEX idx_reply_post ON reply(post_id, replied_to_id);

CREATE TABLE tag (id INTEGER PRIMARY KEY, name TEXT UNIQUE NOT NULL, color TEXT);
CREATE TABLE content_tag (
  content_id TEXT NOT NULL, content_kind TEXT NOT NULL CHECK (content_kind IN ('post','reply')),
  tag_id INTEGER NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
  PRIMARY KEY (content_id, tag_id)
);

CREATE VIRTUAL TABLE content_fts USING fts5(text, content_id UNINDEXED, content_kind UNINDEXED, tokenize='unicode61 remove_diacritics 2');

CREATE TABLE saved_query (id INTEGER PRIMARY KEY, name TEXT NOT NULL, query_json TEXT NOT NULL, created_at INTEGER NOT NULL);

CREATE TABLE asset (
  id TEXT PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('image','video','audio','font')),
  path TEXT NOT NULL,              -- tương đối so với thư mục dữ liệu
  sha256 TEXT NOT NULL,
  license TEXT NOT NULL,           -- SPDX-ish: CC0-1.0 | CC-BY-4.0 | PD | USER-OWNED
  attribution TEXT,                -- bắt buộc nếu CC-BY
  source_url TEXT,
  duration_ms INTEGER, width INTEGER, height INTEGER
);

CREATE TABLE speaker_config (
  id TEXT PRIMARY KEY, label TEXT NOT NULL,       -- "Giọng nữ miền Bắc"
  provider TEXT NOT NULL, voice_id TEXT NOT NULL, -- vd google / vi-VN-Neural2-A
  rate REAL NOT NULL DEFAULT 1.0, pitch REAL NOT NULL DEFAULT 0.0
);

CREATE TABLE project (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, dir TEXT NOT NULL,
  created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, schema_version INTEGER NOT NULL
);

CREATE TABLE render_job (
  id TEXT PRIMARY KEY, project_id TEXT NOT NULL REFERENCES project(id),
  preset TEXT NOT NULL, status TEXT NOT NULL,     -- queued|running|done|failed|cancelled
  progress REAL NOT NULL DEFAULT 0, output_path TEXT,
  plan_hash TEXT, app_version TEXT, ffmpeg_version TEXT, error TEXT,
  started_at INTEGER, finished_at INTEGER
);
```

### 4.2 Quy tắc dữ liệu
- `NULL` ở `like_count/reply_count` nghĩa là **không có dữ liệu thật** (khác `0`). Ranking dựa vào điều này (mục 7).
- Dùng prepared statement cho **mọi** truy vấn; không nối chuỗi SQL.
- Migration tuần tự `NNNN_name.sql`, chạy trong transaction, không sửa migration đã phát hành. Thay đổi schema theo SemVer: breaking → MAJOR, thêm cột tương thích → MINOR, sửa lỗi → PATCH.
- Ghi `schema_version` và `app_version` vào `meta`; trước khi migrate **PHẢI** sao lưu DB (`.bak`), có đường rollback.
- Project file `project.json` mang `schema_version` riêng và có hàm `migrate_project(vN → vN+1)` kèm test.

---

## 5. Tích hợp Threads

### 5.1 Ràng buộc đã biết (cần V-01…V-06 xác nhận)
- API chỉ cho phép đọc **nội dung của tài khoản đã ủy quyền**; không có public feed/post người khác.
- Không có API sửa bài đã đăng (chỉ xoá/đăng lại) → V3 là công cụ **đọc**, không đăng.
- Token long-lived phải refresh định kỳ (≈60 ngày).

### 5.2 Luồng OAuth (desktop)
1. Mở trình duyệt hệ thống tới trang cấp quyền; redirect về `http://127.0.0.1:<random-port>/callback` (máy chủ loopback tạm thời, tắt ngay sau khi nhận code).
2. Đổi `code` → short-lived token → long-lived token; lưu **chỉ trong OS keychain**.
3. Lưu `token_expires_at` vào DB. Cảnh báo UI khi còn ≤ 7 ngày; tự refresh khi còn ≤ 14 ngày nếu app đang mở.
4. `App ID/Secret` do người dùng nhập ở Settings (lưu keychain) hoặc nhúng qua biến môi trường khi build nội bộ — **KHÔNG hardcode**.

### 5.3 Import
```text
list posts (phân trang, since/until) → với mỗi post: lấy replies/conversation (phân trang)
→ upsert (idempotent theo id) → dựng cây (replied_to) → tính depth, is_author_reply
→ cập nhật FTS → ghi last_import_at
```
- **Idempotent:** import lại không tạo trùng; có thể resume từ cursor đã lưu.
- **Rate limiter cục bộ** (token bucket cấu hình được) + backoff mũ khi 429/5xx; hiển thị "còn ~X request" ước lượng.
- Lỗi 401/190 → trạng thái `TokenExpired` → UI: "Token đã hết hạn, vui lòng đăng nhập lại".
- Tiến độ phát qua sự kiện IPC `import://progress {posts_done, replies_done, eta}`.

### 5.4 Fallback offline (BẮT BUỘC có)
Cho phép **nhập thủ công** file JSON (định dạng `testdata/threads_sample.json`) hoặc dán văn bản thread. Cùng pipeline upsert/cây. Mục đích: dev/test không cần mạng, và phương án khi V-06 FAIL.

### 5.5 Cảnh báo UX bắt buộc
Hiển thị: *"Threads không cho sửa bài đã đăng; bản video dùng nội dung tại thời điểm import (`imported_at`)."*

---

## 6. Lọc, tìm kiếm, chọn

### 6.1 Taxonomy bộ lọc
| Nhóm | Tham số |
|---|---|
| Từ khóa | `text_contains` (FTS5), `regex` (tùy chọn, có giới hạn thời gian chạy) |
| Loại | `kind: post/reply`, `has_media`, `is_author_reply`, `depth_min/max` |
| Tác giả | `authors[]`, `exclude_authors[]` |
| Thời gian | `date_from`, `date_to` |
| Tương tác | `min_replies`, `min_likes` (bỏ qua nếu `NULL` và cờ `require_real_metrics=false`) |
| Độ dài | `min_chars`, `max_chars` |
| Nhãn | `tags_any[]`, `tags_all[]`, `tags_none[]` |
| Cảm xúc | `sentiment` (**chỉ** khi người dùng bật; tính cục bộ hoặc gắn tay) |
| Logic | cây `AND/OR/NOT` lồng nhau |

### 6.2 Query model (JSON, versioned)
```json
{
  "version": 1,
  "root": {
    "op": "and",
    "children": [
      { "field": "text", "match": "contains", "value": "ai trong" },
      { "field": "created_at", "match": "between", "value": ["2026-01-01", "2026-12-31"] },
      { "op": "or", "children": [
        { "field": "author", "match": "in", "value": ["userA", "userB"] },
        { "field": "tag", "match": "any", "value": ["highlight"] }
      ]}
    ]
  },
  "sort": { "by": "score_real", "dir": "desc", "fallback": "score_heuristic" },
  "limit": 200
}
```
- `tc-filter` biên dịch JSON → SQL tham số hóa + FTS5 `MATCH`. Có **unit test property-based** (truy vấn rỗng, Unicode có dấu, ký tự đặc biệt).
- Truy vấn lưu được (`saved_query`) và có lịch sử gần đây.

### 6.3 Chọn lựa
- Checkbox từng mục, chọn tất cả kết quả, chọn nhánh (cha → con theo cấu hình `include_children: none|direct|all`).
- Batch: gắn/bỏ tag, thêm vào Project, bỏ chọn. Thao tác batch **undo được**.
- Hiển thị bộ đếm "đã chọn N / tổng M" và thời lượng ước tính khi đọc TTS.

---

## 7. Xếp hạng (Ranking)

### 7.1 Hai chế độ tách bạch
| Chế độ | Điều kiện | Hiển thị nhãn |
|---|---|---|
| **Real-score** | `like_count`/`reply_count` có dữ liệu thật (không `NULL`) | `● Dữ liệu thật` |
| **Heuristic-score** | thiếu dữ liệu thật | `◌ Ước lượng` (kèm tooltip lý do) |

KHÔNG ĐƯỢC trộn hai loại trong cùng một cột xếp hạng mà không gắn nhãn; khi sort hỗn hợp, mục heuristic xếp **sau** mục real trừ khi người dùng chọn "chỉ heuristic".

### 7.2 Công thức Real-score (mặc định, cấu hình được)
```text
engagement = reply_count * w_r + like_count * w_l           # w_r=1.0, w_l=0.5
age_hours  = max(0, (now - created_at) / 3600)
score_real = engagement / (age_hours + 2) ^ gravity          # gravity=1.5
```
- Hằng số nằm trong `ranking.toml`, không hardcode; có preset "Mới nhất", "Phổ biến", "Nhiều phản hồi".

### 7.3 Heuristic-score (không cần metrics)
```text
score_heur = a*len_norm + b*keyword_hits + c*depth_bonus + d*is_author_reply
```
- `len_norm`: chuẩn hóa độ dài về [0,1], có trần (tránh ưu tiên đoạn quá dài).
- `keyword_hits`: từ khóa người dùng đặt trong Settings.
- Mọi hệ số mặc định `a=0.4, b=0.3, c=0.1, d=0.2`, cấu hình được.
- Sentiment (nếu bật) là **một nhãn lọc**, không tự cộng điểm trừ khi người dùng bật hệ số `e`.

### 7.4 Minh bạch
Mỗi mục có popover "Vì sao điểm này?" liệt kê từng thành phần (giá trị × hệ số = đóng góp) và loại điểm. Có trang Help giải thích công thức kèm ví dụ số. Unit test kiểm tra: thứ tự ổn định, tie-break theo `created_at` rồi `id`.

---

## 8. UX / Màn hình

### 8.1 Creator workflow (11 bước)
Login → Import → Review & Lọc → Thêm vào Project → Editor/Timeline → Assets → Chỉnh timeline → Giọng & caption → Âm thanh → Preview → Render Queue → History.

### 8.2 Danh sách màn hình
| ID | Màn hình | Thành phần chính | Trạng thái bắt buộc |
|---|---|---|---|
| S01 | Dashboard/Projects | danh sách project, tạo/mở/nhân bản | rỗng, đang tải, lỗi |
| S02 | Account & Import | trạng thái token, nút đăng nhập, tiến độ import, log | chưa đăng nhập, đang import, hết token, lỗi mạng |
| S03 | Content Library | bảng/cây post-reply (trái), panel lọc (phải), "Add to Project" | rỗng, kết quả 0, hàng nghìn mục (virtualized) |
| S04 | Assets Library | tab Backgrounds / Memes / Audio / Fonts; thumbnail + license badge | thiếu license → chặn dùng |
| S05 | Editor/Timeline | track Video/Text/Image/Audio, ruler theo frame, toolbar | chọn nhiều, snap, khóa track |
| S06 | Preview | phát/dừng/seek, zoom timeline, chụp thumbnail | giật khung → cảnh báo |
| S07 | Voice & Caption | form văn bản, chọn speaker, nghe thử, bảng caption chỉnh timing | `timing_estimated` có cờ |
| S08 | Render Queue | preset, tiến độ, hủy, mở thư mục | queued/running/failed/done |
| S09 | History | video đã xuất + metadata (ngày, app_version, plan_hash) | — |
| S10 | Settings | ngôn ngữ, thư mục dữ liệu, TTS mặc định, ranking, hiệu năng, log | — |
| S11 | Notifications/Logs | thanh trạng thái + trình xem log | — |

### 8.3 Yêu cầu chung
- Dark mode mặc định + light; theme nhất quán, không màu chói.
- **Undo/redo** vô hạn trong phiên cho mọi thao tác timeline/chọn lựa (command pattern).
- Tự lưu project (autosave 30 s + khi thao tác lớn), chỉ báo "đã lưu".
- Phím tắt (có thể đổi): `Space` phát/dừng · `J/K/L` lùi/dừng/tiến · `S` tách clip · `Del` xoá · `Ctrl+Z/Y` · `Ctrl+S` · `←/→` ±1 frame · `Shift+←/→` ±10 frame · `Ctrl+K` command palette.
- Thông báo lỗi: nêu *chuyện gì xảy ra – vì sao – làm gì tiếp*; không lộ stacktrace với người dùng (chỉ trong dev mode).

---

## 9. Timeline & định dạng project

### 9.1 Thời gian
- Đơn vị gốc: **frame số nguyên**. `fps` là số hữu tỉ (`30/1`, `60/1`, `30000/1001`). KHÔNG dùng float giây để lưu vị trí/độ dài.
- Hàm đổi `frame ↔ ms ↔ timecode` nằm trong `tc-core::time`, có test vòng (round-trip) với mọi fps hỗ trợ.

### 9.2 Cấu trúc `project.json` (rút gọn)
```json
{
  "schema_version": 1,
  "canvas": { "width": 1080, "height": 1920, "fps": "30/1", "background": "#000000" },
  "tracks": [
    { "id": "t-video", "kind": "video", "locked": false, "clips": [
      { "id": "c1", "asset_id": "a-bg1", "start": 0, "duration": 900, "in": 0, "loop": true }
    ]},
    { "id": "t-text", "kind": "text", "clips": [
      { "id": "c2", "content_ref": {"kind":"reply","id":"178..."}, "start": 30, "duration": 120,
        "style_id": "card-default", "transition_in": {"type":"fade","frames":6} }
    ]},
    { "id": "t-voice", "kind": "audio", "role": "voice", "clips": [
      { "id": "c3", "tts": {"speaker_id":"sp-nu-bac","text":"...", "audio_asset_id":"a-tts-1"},
        "start": 30, "duration": 118 }
    ]},
    { "id": "t-music", "kind": "audio", "role": "music", "ducking": {"enabled": true, "reduce_db": 12} , "clips": [] }
  ],
  "captions": { "format": "srt", "cues": [] },
  "seed": 12345
}
```
- Validate bằng JSON Schema (`docs/spec/project.schema.json`) khi mở/lưu; lỗi → báo rõ clip nào, trường nào.
- `content_ref` trỏ về bài/comment; **chụp snapshot** văn bản vào project để video không đổi khi dữ liệu nguồn đổi.

### 9.3 Thao tác chỉnh sửa (đều là Command, undo được)
`AddClip · MoveClip · TrimClip · SplitClip · DeleteClip · SetTransition · SetStyle · AddTrack · LockTrack · BatchTag …`
- Quy tắc: không chồng lấp trong cùng track (trừ track `overlay`), snap vào frame/biên clip/playhead (bật/tắt được).
- Preset **Thẻ văn bản** (card style): font, cỡ chữ, padding, màu nền, avatar tùy chọn. Cỡ chữ tự co để vừa khung (tối thiểu 36 px ở 1080×1920) – nếu không vừa, chia thẻ.

### 9.4 Chỉ đạo chuyển động (Motion direction)
- Mỗi preset chuyển cảnh: `fade`, `slide-up`, `pop`, `cut`; thời lượng 4–12 frame; easing `ease-out-cubic` mặc định.
- Hiệu ứng ngẫu nhiên (nếu có) **PHẢI** lấy từ `seed` của project (PRNG cố định) để render lặp lại giống nhau.
- Tôn trọng `prefers-reduced-motion` ở **UI** (không áp dụng lên video xuất).

### 9.5 Gói meme gốc
- Thư mục `assets/builtin/memes/` chỉ gồm hình **tự tạo hoặc CC0**; mỗi file có dòng trong `LICENSES.md` + bản ghi `asset.license`.
- Import ngoài: bắt buộc điền `license` (CC0/CC-BY/PD/USER-OWNED) và `source_url`; CC-BY **bắt buộc** `attribution` (tự động chèn vào mô tả xuất / file `credits.txt`).

---

## 10. Âm thanh, TTS, phụ đề

### 10.1 Provider TTS (trait `TtsProvider`)
```rust
trait TtsProvider {
  fn list_voices(&self) -> Result<Vec<Voice>>;
  fn synthesize(&self, req: TtsRequest) -> Result<TtsResult>; // audio + (tuỳ chọn) timepoints
}
```
- Provider tối thiểu: (a) **Mock** (tiếng bíp/sine cho test), (b) 1 provider đám mây đã qua V-07, (c) 1 provider offline nếu V-08 PASS.
- Có **cache** theo `hash(text, voice_id, rate, pitch, provider_version)` → file audio; tái dùng khi render lại.
- API key lưu keychain; lỗi mạng → thử lại có backoff, cuối cùng báo rõ và cho phép "bỏ qua voice".
- KHÔNG clone giọng, KHÔNG upload audio mẫu của người thật.

### 10.2 Caption
- Nguồn timing theo thứ tự ưu tiên: **timepoint từ TTS** → forced alignment cục bộ → ước lượng theo tốc độ nói (gắn cờ `timing_estimated=true`, hiển thị cảnh báo ở S07).
- Quy tắc cue: ≤ 2 dòng, ≤ 42 ký tự/dòng, thời lượng 1–6 s, không vượt biên clip. Xuất `.srt` **và** `.vtt`; có tuỳ chọn burn-in vào video.
- Font hiển thị PHẢI hỗ trợ đầy đủ dấu tiếng Việt (test với chuỗi mẫu `ẳ ỗ ự ợ ề`).

### 10.3 Ma trận đánh giá TTS tiếng Việt (chạy ở M3-T01)
| Tiêu chí | Cách đo | Ngưỡng chấp nhận |
|---|---|---|
| Độ rõ/đúng dấu | 20 câu mẫu (số, tên riêng, viết tắt, tiếng lóng) – 3 người nghe chấm 1–5 | trung bình ≥ 3.5 |
| Độ trễ | thời gian synth câu 15 từ | cloud ≤ 1.5 s (p50) ; offline ≤ 3 s |
| Có timepoint | có/không | ưu tiên có |
| Chi phí | giá/1k ký tự (ghi nguồn + ngày) | báo cáo, không ngưỡng cứng |
| Giấy phép | điều khoản dùng thương mại/đăng lại | rõ ràng, ghi vào `docs/verification/` |
Kết quả điền `docs/verification/tts-matrix.md`; **không** hứa độ trễ "<500 ms" nếu chưa đo.

### 10.4 Mix & ducking
- Chuẩn nội bộ: 48 kHz, stereo, float; resample đầu vào bằng `aresample`.
- Loudness mục tiêu: **−14 LUFS tích hợp** (xuất), true-peak ≤ −1 dBTP (dùng `loudnorm` hai lượt).
- Ducking: nhạc giảm **10–15 dB** khi có voice, attack ~50 ms, release ~300 ms. Ví dụ graph:
```text
[voice]asplit=2[v_main][v_sc];
[music][v_sc]sidechaincompress=threshold=0.05:ratio=8:attack=50:release=300[music_duck];
[music_duck][v_main]amix=inputs=2:normalize=0[mix]
```
(Giá trị tinh chỉnh bằng đo thực tế; ghi lại trong `docs/qa/audio-ducking.md`.)
- SFX gắn sự kiện (`scene_start`, `scene_end`, `transition`) và có thể bật/tắt toàn cục.

---

## 11. Renderer

### 11.1 Quy trình
```text
project.json ─validate→ RenderPlan (bất biến, có plan_hash)
  → tạo filter-graph / raster từng frame → FFmpeg sidecar → MP4 (H.264 + AAC)
  → verify (ffprobe: fps, số frame, thời lượng, codec) → ghi render_job
```
- `plan_hash = sha256(canonical(project) + assets sha256 + preset + seed + renderer_version)`; lưu cùng output.
- Render chạy trong **tiến trình riêng**, có thể **hủy**, trả tiến độ ra IPC, ghi file tạm `*.partial` rồi đổi tên khi xong.

### 11.2 Preset xuất
| Preset | Độ phân giải | FPS | Video | Audio |
|---|---|---|---|---|
| `shorts-1080x1920` (mặc định) | 1080×1920 | 30 | H.264 High, CRF 18, `yuv420p`, `+faststart` | AAC 192 kbps |
| `shorts-720x1280` | 720×1280 | 30 | CRF 20 | AAC 128 kbps |
| `draft-540x960` | 540×960 | 30 | CRF 26, `veryfast` | AAC 96 kbps |
- GPU encode (NVENC/QSV/AMF) là **tuỳ chọn**, mặc định tắt vì không đảm bảo determinism pixel.

### 11.3 Frame-accurate & deterministic
- **Định nghĩa:** với cùng `plan_hash`, `ffmpeg_version`, CPU encoder (`libx264` cố định `-threads` & preset) → **mọi frame giải mã ra giống hệt về pixel** (so sánh bằng hash từng frame sau decode), dù bitstream có thể khác.
- Số frame đầu ra = `sum(duration)` theo plan, sai lệch **0 frame**. Âm thanh dài bằng video ±1 frame audio.
- Hiệu ứng ngẫu nhiên dùng `seed` của project; cấm đọc đồng hồ/ngẫu nhiên hệ thống trong đường render.
- Pin phiên bản FFmpeg (ghi trong `render_job.ffmpeg_version`); CI tải đúng bản đã pin.

### 11.4 Bộ kiểm thử Golden
- `testdata/golden/` gồm ≥ 3 project mẫu (chỉ text; text+meme+voice mock; đủ tính năng) và file `expected_frames.json` (hash các frame mốc: 0, giữa, cuối, và frame tại mỗi transition).
- Test e2e: render → `ffprobe` → so khớp số frame/fps/codec → so hash frame mốc. Lệch → FAIL và đính kèm ảnh diff vào artifact.
- Benchmark: clip 5 phút 1080×1920 trên "máy chuẩn thấp" (ghi cấu hình CPU/RAM) – ghi thời gian vào `docs/qa/perf.md`; không đặt ngưỡng cứng trước khi có số đo thực.

### 11.5 Preview
- Preview dùng pipeline nhẹ (độ phân giải thấp, có thể bỏ qua một số hiệu ứng nặng) nhưng **cùng mô hình thời gian**; mục tiêu ≥ 30 fps ổn định (60 fps khi máy cho phép).
- Giật/khuyết khung → log + cảnh báo S06.

---

## 12. Lưu trữ, cache, phục hồi

- **Thư mục dữ liệu** (cấu hình được): `data/app.db`, `data/projects/<id>/{project.json, media/, cache/, renders/}`, `data/logs/`.
- File lớn **không** nằm trong DB; chỉ lưu đường dẫn tương đối + `sha256`. Khi thêm asset vào project: **copy** vào `media/` để project tự đủ.
- **Export project** (`.tcproj` = zip: `project.json` + `media/` + `db-subset.json`) và import trên máy khác; kiểm `sha256` khi nhập.
- DB: SQLite **WAL**, `PRAGMA foreign_keys=ON`, mọi thao tác ghi nhiều bước nằm trong transaction.
- **Crash recovery:** khi mở app, quét `render_job.status='running'` → đánh dấu `interrupted`, hỏi *Resume* (nếu hỗ trợ) hoặc *Hủy & dọn file `.partial`*. Autosave giữ `project.json.bak` gần nhất.
- Cache API (kết quả list posts) TTL mặc định 24 h, khoá theo `account_id`; nút "Làm mới" bỏ qua cache. Cache TTS xem 10.1. Có nút "Dọn cache" + giới hạn dung lượng (mặc định 5 GB, LRU).

---

## 13. Lỗi & quan sát

- Mọi lỗi dùng enum có mã (`E_TOKEN_EXPIRED`, `E_RATE_LIMITED`, `E_RENDER_FFMPEG`, …) + thông điệp người dùng + gợi ý hành động.
- Logging: `tracing` → file JSON có xoay vòng (10 file × 10 MB); mức `ERROR/WARN/INFO/DEBUG`. **Bộ lọc che (redaction)** loại token, email, header `Authorization` trước khi ghi; có unit test cho redaction.
- Nút "Xuất gói chẩn đoán" (log đã che + phiên bản + cấu hình, **không** kèm nội dung bài) để người dùng gửi khi báo lỗi — chỉ tạo file cục bộ, không tự gửi.
- Telemetry: **tắt** mặc định; nếu thêm sau này phải opt-in + ADR riêng.

---

## 14. Bảo mật, riêng tư, tuân thủ

### 14.1 Threat model (STRIDE rút gọn)
| Mối đe dọa | Biện pháp | Kiểm chứng |
|---|---|---|
| Lộ token/secret | keychain; redaction log; secret scanning CI (gitleaks) | test redaction; CI xanh |
| SQL injection | prepared statements, không nối chuỗi | test với payload `'; DROP--` |
| Đường dẫn độc (zip-slip khi import `.tcproj`) | chuẩn hóa & chặn `..`, kiểm `sha256` | test file zip độc |
| Lệnh FFmpeg bị chèn | KHÔNG dùng shell; truyền tham số dạng mảng; whitelist filter | review + test |
| Nội dung WebView/XSS (UI hiển thị text người dùng) | render text bằng React (escape mặc định), CSP chặt trong Tauri, tắt `eval` | kiểm CSP |
| Loopback OAuth bị chiếm cổng | cổng ngẫu nhiên, `state` ngẫu nhiên, đóng sau lần dùng | test |
| Phụ thuộc có lỗ hổng | `cargo audit`/`cargo deny` + `npm audit` trong CI | CI |

### 14.2 Riêng tư (GDPR-aware)
- Dữ liệu cá nhân (username, nội dung) chỉ ở máy; có chức năng **xóa tài khoản & dữ liệu** (xóa DB rows + file media liên quan + token trong keychain).
- Nội dung của người khác xuất hiện trong reply: hiển thị `@username` mặc định có tuỳ chọn **ẩn/che tên** trong video.
- Tài liệu `docs/PRIVACY.md` nêu rõ dữ liệu nào gọi ra ngoài (Threads API, TTS cloud) và khi nào.
- Khi dùng TTS cloud: cảnh báo rõ "văn bản sẽ được gửi tới nhà cung cấp"; có chế độ **chỉ offline**.

### 14.3 Bản quyền & giọng nói
- Xem GR-04, GR-05, mục 9.5. UI chặn dùng asset thiếu license; xuất `credits.txt` tự động cho CC-BY.

### 14.4 Giấy phép phần mềm
- Mã nguồn: MIT hoặc Apache-2.0 (chọn bằng ADR). Thêm `cargo-deny` kiểm license phụ thuộc.
- **FFmpeg:** dùng build LGPL (không `--enable-gpl`/`libx264` nếu muốn tránh GPL) *hoặc* chấp nhận GPL và công bố nghĩa vụ; **quyết định bằng ADR-0003** trước khi phát hành. Kèm thông báo license trong bản cài.

---

## 15. Khả năng tiếp cận & đa ngôn ngữ

- WCAG 2.2 AA: tương phản text ≥ 4.5:1 (UI lớn ≥ 3:1), focus nhìn thấy rõ, mọi control có nhãn, thứ tự Tab hợp lý, `Esc` đóng hộp thoại.
- **100% chức năng chính dùng được bằng bàn phím** (test thủ công theo checklist + `axe-core` cho UI).
- Cỡ chữ UI chỉnh được (100–200%); không truyền đạt thông tin bằng màu đơn thuần.
- i18n: chuỗi trong `ui/src/locales/{vi,en}.json` (khóa ổn định, không hardcode chuỗi trong component); định dạng ngày/số theo locale.
- Video xuất có caption (`.srt/.vtt`) theo mặc định; ảnh meme có `alt` trong metadata.

---

## 16. Ngân sách hiệu năng (đề xuất, hiệu chỉnh sau khi đo)

| Hạng mục | Mục tiêu |
|---|---|
| Lọc/tìm kiếm trên 50k mục | p95 ≤ 100 ms |
| Mở project 200 clip | ≤ 2 s |
| Kéo-thả/scrub timeline | ≥ 30 fps (mục tiêu 60) |
| Import 100 post (không tính thời gian mạng) | xử lý cục bộ ≤ 5 s |
| Bộ nhớ khi idle | ≤ 400 MB |
| Khởi động app | ≤ 3 s trên máy chuẩn thấp |
Mỗi số liệu có *benchmark test* trong `docs/qa/perf.md`; số nào chưa đo ghi `UNMEASURED`.

---

## 17. GitHub automation & giao thức agent

### 17.1 Nhánh & commit
- **Trunk-based:** `main` luôn release-ready; nhánh ngắn `feat/*`, `fix/*`, `chore/*`, `docs/*`, `ci/*`. Merge **chỉ qua PR** (squash).
- **Conventional Commits:** `type(scope): mô tả` – `feat|fix|perf|refactor|docs|test|build|ci|chore`; breaking dùng `!` hoặc footer `BREAKING CHANGE:`. `scope` ∈ tên crate/màn hình (`threads`, `render`, `ui-timeline`…). Kiểm bằng `commitlint` trong CI.
- SemVer: `fix`→PATCH, `feat`→MINOR, breaking→MAJOR; tag `vX.Y.Z`.
- **Cấm:** force-push, rewrite lịch sử `main`, commit file > 10 MB (dùng Git LFS cho golden video), commit secret.

### 17.2 CI (GitHub Actions) – workflow chuẩn
```yaml
name: CI
on:
  push: { branches: [main] }
  pull_request: { branches: [main] }
permissions: { contents: read }
concurrency: { group: ci-${{ github.ref }}, cancel-in-progress: true }

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: "rustfmt, clippy" }
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - uses: actions/setup-node@v4
        with: { node-version: 20, cache: npm, cache-dependency-path: ui/package-lock.json }
      - run: npm ci --prefix ui
      - run: npm run lint --prefix ui && npm run typecheck --prefix ui

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-deny cargo-audit --locked
      - run: cargo deny check && cargo audit
      - uses: gitleaks/gitleaks-action@v2

  test:
    needs: lint
    strategy:
      fail-fast: false
      matrix: { os: [ubuntu-latest, windows-latest, macos-latest] }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
        with: { lfs: true }
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace --all-features
      - run: npm ci --prefix ui && npm test --prefix ui

  e2e-render:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { lfs: true }
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Cài FFmpeg đúng phiên bản đã pin
        run: ./scripts/install-ffmpeg-pinned.sh
      - run: cargo test -p tc-render --test golden -- --nocapture
      - uses: actions/upload-artifact@v4
        if: always()
        with: { name: e2e-render-report, path: "docs/qa/out/**" }
```
- Ghim version action theo **tag/commit SHA** đã kiểm tra; Dependabot cập nhật hằng tuần. Các số phiên bản trong YAML là mẫu — agent **PHẢI** xác nhận phiên bản hiện hành trước khi dùng.
- `release.yml`: kích hoạt khi tag `v*.*.*` → build Tauri bundle 3 OS → tạo GitHub Release (notes từ CHANGELOG) → đính kèm artifact + checksum `SHA256SUMS` + SBOM (`cargo-cyclonedx`/`syft`). Ký số khi khả thi; nếu chưa có chứng chỉ thì ghi rõ **"unsigned"** trong notes.
- Nếu build lỗi → **không** tạo release; nếu tag đã lỡ tạo, xoá tag bằng thao tác được cấp quyền rồi sửa (không force-push).

### 17.3 Cấp quyền cho agent
| Cấp | Được phép | Cần gì |
|---|---|---|
| **L0** | Đọc repo, đề xuất patch trong chat | — |
| **L1** (mặc định) | Sửa code cục bộ, chạy test/build/render, tạo báo cáo QA, **commit cục bộ** trên nhánh riêng | — |
| **L2** | `git push` nhánh `feat/*` `fix/*` và mở **PR** | Người dùng ghi rõ "cho phép push/PR" trong phiên |
| **L3** | Tạo tag/release, cập nhật workflow quyền cao | Người dùng ghi rõ từng lần + PR đã được review |
Nếu không chắc cấp nào đang có → **coi là L1** và hỏi.

### 17.4 Vòng lặp cho mỗi task (bắt buộc)
```text
1. Đọc task (ID, acceptance, bằng chứng yêu cầu) + kiểm tra git status sạch
2. Tạo nhánh  feat/<id>-<slug>
3. Viết test trước/cùng code; implement tối thiểu để pass
4. Chạy: fmt → clippy → test → (nếu liên quan) render golden → UI lint/test
5. Tự review diff; cập nhật docs/ADR/CHANGELOG nếu cần
6. Commit theo Conventional Commits (nhỏ, có thể kiểm chứng)
7. Sinh docs/qa/<task-id>.md  (mẫu bên dưới)
8. Nếu có quyền L2: push + mở PR kèm báo cáo; nếu không: dừng và báo người dùng
```

**Mẫu báo cáo QA (`docs/qa/<task-id>.md`):**
```markdown
# <TASK-ID> – <tên>
- Commit: <sha>  | Nhánh: <tên>  | Ngày: <YYYY-MM-DD>
- Kết quả: PASS | FAIL | BLOCKED | UNVERIFIED
## Lệnh đã chạy & kết quả
| Lệnh | Kết quả | Ghi chú |
## Tiêu chí nghiệm thu
| ID | Đạt? | Bằng chứng (đường dẫn log/ảnh/video) |
## Việc chưa làm / rủi ro còn lại
```

### 17.5 Điều kiện dừng (agent PHẢI dừng và báo người dùng)
- Test lỗi mà không sửa được trong phạm vi task; spike (V-xx) FAIL ảnh hưởng thiết kế.
- Cần secret/quyền chưa có; cần cài thứ gì ngoài phạm vi repo.
- Phát hiện yêu cầu mâu thuẫn guardrails (mục 1.3).
- Thao tác sẽ ghi đè/xoá dữ liệu người dùng hoặc lịch sử git.

### 17.6 Bảo vệ nhánh & rollback
- `main` bật branch protection: ≥ 1 review, status checks `lint/security/test/e2e-render` phải xanh, yêu cầu nhánh up-to-date, cấm force-push, giải quyết hết conversation.
- Rollback: `git revert` commit lỗi → PR → bản PATCH/hotfix; release lỗi → đánh dấu *pre-release/yanked* và phát bản mới (không viết lại lịch sử).

---

## 18. Lộ trình & task (Milestones)

> Quy ước ID: `M<số milestone>-T<số task>`. Mỗi task kết thúc bằng báo cáo QA (17.4).

### M0 – Nền móng (Repo & quy trình)
| ID | Task | Nghiệm thu (Given/When/Then) | Bằng chứng |
|---|---|---|---|
| M0-T01 | Khởi tạo workspace, Tauri app rỗng, UI Vite, `AGENTS.md`, template PR/Issue | *Given* clone mới *When* `cargo build && npm ci && npm run build` *Then* thành công trên 3 OS | log CI |
| M0-T02 | CI chuẩn (17.2) + commitlint + Dependabot + CODEOWNERS | *Given* PR mẫu *When* CI chạy *Then* tất cả job xanh; commit sai format bị chặn | link run |
| M0-T03 | Viết ADR 0001–0006 | *Then* 6 file ADR `Accepted`, đúng template | file |
| M0-T04 | Spike V-01…V-11 (làm dần, ghi từng file) | *Then* mỗi V-xx có kết quả + fallback đã chọn | `docs/verification/` |

### M1 – MVP dữ liệu (Phase 1)
| ID | Task | Nghiệm thu | Bằng chứng |
|---|---|---|---|
| M1-T01 | `tc-db`: schema v1 + migration + FTS5 + backup trước migrate | *Given* DB v0 *When* migrate *Then* dữ liệu giữ nguyên, `schema_version=1`; rollback khôi phục được | test log |
| M1-T02 | Import **offline** từ JSON mẫu + dựng cây | *Given* `threads_sample.json` (≥100 post, reply lồng ≥4 cấp) *When* import hai lần *Then* không trùng; `depth` đúng 100% | test |
| M1-T03 | `tc-threads`: OAuth loopback + list posts + replies + rate limiter (mock server) | *Given* mock trả 429 *When* import *Then* backoff, không vượt rate; 401 → `E_TOKEN_EXPIRED` | test + log |
| M1-T04 | `tc-filter` + UI S03 (lọc cơ bản, chọn, Add to Project) | *Given* 50k mục *When* tìm "ai trong" *Then* kết quả đúng, p95 ≤ 100 ms | benchmark |
| M1-T05 | `tc-timeline` mô hình + undo/redo + validate schema | *Given* chuỗi 100 lệnh *When* undo hết *Then* trạng thái = ban đầu (property test) | test |
| M1-T06 | Editor tối thiểu (text card trên nền tĩnh) + preview | *Then* kéo-thả, trim, snap frame; preview ≥ 30 fps | video quay màn hình/ảnh |
| M1-T07 | `tc-render`: MP4 H.264 không voice + golden test #1 | *Then* số frame khớp 0 sai lệch; hash frame mốc khớp | artifact e2e |

### M2 – Âm thanh (Phase 2)
| ID | Task | Nghiệm thu |
|---|---|---|
| M2-T01 | Trait TTS + Mock + provider đã qua V-07 + cache | cùng input → dùng cache, không gọi mạng lần 2 |
| M2-T02 | Voice & Caption (S07) + SRT/VTT | cue hợp quy tắc 10.2; có cờ `timing_estimated` khi ước lượng |
| M2-T03 | Assets Library (S04) + kiểm license | thêm asset thiếu license → bị chặn; CC-BY sinh `credits.txt` |
| M2-T04 | Mix + ducking + loudnorm | nhạc giảm 10–15 dB khi có voice (đo bằng script), −14 LUFS ±1 |
| M2-T05 | Golden test #2 (text+meme+voice mock) | khớp số frame/hash mốc; A/V lệch ≤ 1 frame |

### M3 – Nâng cao (Phase 3)
| ID | Task | Nghiệm thu |
|---|---|---|
| M3-T01 | Ma trận TTS (10.3) + chọn provider mặc định bằng ADR | file `tts-matrix.md` đầy đủ |
| M3-T02 | Ranking real/heuristic + popover giải thích | unit test thứ tự; nhãn đúng; tie-break ổn định |
| M3-T03 | Transitions, presets thẻ, speaker presets | golden #3 đạt |
| M3-T04 | Video demo hoàn chỉnh có voice + nhạc + caption | video mẫu 30–60 s đính kèm QA |

### M4 – Hoàn thiện & phát hành (Phase 4)
| ID | Task | Nghiệm thu |
|---|---|---|
| M4-T01 | Render queue (S08), resume/hủy, crash recovery | giết tiến trình giữa chừng → mở lại xử lý đúng |
| M4-T02 | Accessibility + i18n vi/en | checklist bàn phím đạt; `axe` không lỗi nghiêm trọng |
| M4-T03 | Hiệu năng (bảng 16) trên 3 OS | số đo ghi `perf.md`; mục không đạt có kế hoạch |
| M4-T04 | Đóng gói + `release.yml` + SBOM + checksum + tài liệu hướng dẫn | tạo `v3.0.0` (cần quyền L3) |

---

## 19. Ma trận nghiệm thu & Definition of Done

| Hạng mục | Tiêu chí (Có/Không) | Chỉ số |
|---|---|---|
| Import | Lấy được cây reply đầy đủ; token hết hạn không panic | lỗi API < 5% (mock) ; xử lý cục bộ ≤ 5 s/100 post |
| Filter | Kết quả đúng theo từ khóa/tag/tác giả/ngày/logic | ≥ 95% bộ query test đúng; p95 ≤ 100 ms |
| Ranking | Nhãn real/heuristic đúng; công thức hiển thị được | unit test pass; thứ tự ổn định |
| Timeline | Kéo-thả, trim, snap, undo/redo, preview đúng | property test undo; preview ≥ 30 fps |
| TTS & Audio | Voice tiếng Việt đúng mapping; SRT đồng bộ; ducking đúng | ducking 10–15 dB; A/V lệch ≤ 1 frame; −14 LUFS ±1 |
| Render | MP4 phát được; 0 frame lệch; deterministic | hash frame mốc khớp golden |
| UI/UX | Đủ màn hình S01–S11, đủ trạng thái rỗng/lỗi | checklist UX đạt |
| Accessibility | Dùng được hoàn toàn bằng bàn phím; tương phản đạt | axe + checklist |
| Bảo mật | Không lộ secret; redaction; deny/audit sạch | gitleaks/cargo-deny/audit xanh |
| CI/CD | PR chạy CI 3 OS xanh; commit hợp lệ; branch protection bật | 100% run xanh trên `main` |

**Definition of Done cho mỗi task/issue:**
1. Code + test đã merge vào `main` qua PR đã review; commit đúng Conventional Commits.
2. fmt/clippy/test (và golden nếu liên quan) **xanh trong CI** — có link run.
3. Docs/ADR/CHANGELOG/spec cập nhật nếu hành vi đổi.
4. Có báo cáo QA `docs/qa/<id>.md` với bằng chứng thật (log/ảnh/video mẫu nếu là tính năng nhìn/nghe được).
5. Coverage không giảm dưới ngưỡng (khởi đầu 70% cho crate logic; nâng dần).
6. Không còn lỗi crash/cảnh báo bảo mật mở ở mức cao.

---

## 20. Thay đổi V2 → V3

- Thêm: schema + migration + versioning; ranh giới module; ADR; luồng/màn hình chi tiết; taxonomy lọc & query model; ranking tách real/heuristic; ma trận TTS; yêu cầu frame-accurate/deterministic + golden test; lưu trữ/cache/recovery; accessibility/i18n; quan sát lỗi; threat model/riêng tư/license; lộ trình có tiêu chí đo; GitHub automation + giao thức agent.
- **Chỉnh so với báo cáo nghiên cứu thô:** (1) đưa mọi khẳng định chưa kiểm chứng vào **Sổ xác minh** thay vì coi là thật; (2) tách `threads_publishing_limit` (hạn mức đăng) khỏi hạn mức đọc; (3) bỏ `actions-rs/toolchain` (không còn được bảo trì) → `dtolnay/rust-toolchain`; (4) FFmpeg dạng sidecar + ADR license; (5) thêm fallback import offline; (6) thêm cấp quyền L0–L3 cho agent; (7) bỏ các ngưỡng chưa đo (vd TTS "<500 ms") → đo trước, ghi sau.

## 21. Câu hỏi mở (chờ người dùng quyết định)

1. Giữ **Tauri + React** hay muốn native thuần Rust (egui)? *(mặc định: Tauri)*
2. Nền tảng ưu tiên phát hành đầu tiên? *(mặc định: Windows)*
3. Có sẵn **Meta App ID/Secret** để chạy spike OAuth thật chưa, hay dùng offline JSON trước?
4. TTS mặc định: cloud (Google) hay offline? Có ngân sách API không?
5. Giấy phép mã nguồn: MIT hay Apache-2.0? Repo public hay private?
6. Cấp quyền agent hiện tại là L1 hay L2?

## 22. Tham khảo (agent phải tra bản mới nhất trước khi dùng)

Threads API (Meta for Developers) · Conventional Commits · Semantic Versioning · GitHub Actions & branch protection docs · Tauri 2 docs · SQLite FTS5 · FFmpeg (filter `sidechaincompress`, `loudnorm`, `aresample`) · Google Cloud Text-to-Speech (danh sách giọng `vi-VN`) · WCAG 2.2 · Architecture Decision Records (M. Nygard / M. Fowler) · RustSec / cargo-deny.
