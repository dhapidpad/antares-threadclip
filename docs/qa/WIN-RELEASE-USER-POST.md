# QA Report – Windows Build & User Threads Post Integration Test

- **Date:** 2026-11-19
- **Author:** Antigravity AI Agent
- **Target Post:** [https://www.threads.com/@cung_khong_iu_toan_lam/post/Df5D-Hak-d3](https://www.threads.com/@cung_khong_iu_toan_lam/post/Df5D-Hak-d3)
- **Repository:** [https://github.com/dhapidpad/antares-threadclip](https://github.com/dhapidpad/antares-threadclip)
- **Build Windows Workflow Run:** [44474959114](https://github.com/dhapidpad/antares-threadclip/actions/runs/44474959114) (**PASSED**)
- **CI Workflow Run:** [44474959085](https://github.com/dhapidpad/antares-threadclip/actions/runs/44474959085) (**PASSED**)

---

## 1. Windows Release Pipeline Verification

Workflow: `.github/workflows/build-windows.yml`
Runner: `windows-latest` (Windows Server 2022 / x86_64)

### Artifact Package: `antares-threadclip-windows-x64`
Package structure verified:
```
antares-threadclip-windows-x64/
├── tc-app.exe                    # Compiled Rust core / IPC server (release mode)
├── bin/
│   └── ffmpeg.exe                # Bundled standalone FFmpeg 7.x sidecar
├── ui/                           # Production web bundle (React 18 + Vite)
│   ├── index.html
│   └── assets/
├── assets/                       # Bundled CC0 media assets & sample B-roll
├── run.bat                       # One-click portable launcher
└── SHA256SUMS.txt                # Cryptographic verification checksums
```

### Local Build Script
Also provided for local Windows environments:
- `scripts/build-windows.ps1` (automates cargo release build, UI vite build, and sidecar assembly).

---

## 2. Live Threads Post Test & Verification

- **Post URL:** `https://www.threads.net/@cung_khong_iu_toan_lam/post/Df5D-Hak-d3`
- **Post ID:** `Df5D-Hak-d3`
- **Author:** `@cung_khong_iu_toan_lam`
- **Extracted Structure:**
  - 1 Root Post + 14 Replies (including nested reply chains up to depth 4).
  - Identified author response nodes with author reply bonus flag.
- **Fixture File:** `testdata/test_user_thread_Df5D.json`
- **End-to-End Automated Test:** `crates/tc-app/tests/test_threads_user_post.rs`

### Test Pipeline Checklist:
1. **Database Import (tc-db):** Thread and all 14 replies ingested into SQLite with FTS5 indexing.
2. **Reply Tree Reconstruction (tc-threads):** Hierarchical tree built with root post and multi-branch replies, correctly marking author interaction nodes.
3. **Dual Scoring (tc-ranking):** Real engagement scores and heuristic scoring computed; author response heuristic factor `"Phản hồi của tác giả"` positively scored.
4. **Timeline Generation (tc-timeline):** Multi-track vertical (9:16, 1080x1920) timeline created, sequencing video, audio, and captions.
5. **TTS & Subtitle Generation (tc-audio):** TTS audio WAV and matching SRT subtitle cues generated.
6. **FFmpeg Command Builder (tc-render):** Deterministic filter complex and command vector constructed targeting vertical video specifications.

---

## 3. GitHub Actions Verification

Both pipelines completed successfully on `main` branch commit `80e7b95`:
- **Build Windows Release:** Run ID `44474959114` (Success in 1m52s)
- **CI Matrix (Ubuntu, macOS, Windows x64, e2e-render):** Run ID `44474959085` (Success in 2m13s)
