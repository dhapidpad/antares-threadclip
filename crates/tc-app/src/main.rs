use std::path::PathBuf;
use std::sync::Arc;
use tc_app::{server::StudioServer, AppController, AppState};

fn find_ui_dir() -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let p1 = exe_dir.join("ui");
            if p1.exists() {
                return p1;
            }
        }
    }
    let p2 = PathBuf::from("ui/dist");
    if p2.exists() {
        return p2;
    }
    let p3 = PathBuf::from("ui");
    if p3.exists() {
        return p3;
    }
    PathBuf::from("ui")
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let app_arg = format!("--app={}", url);
        let candidates = [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        ];

        let mut opened_in_app_mode = false;
        for path in &candidates {
            if std::path::Path::new(path).exists() {
                if std::process::Command::new(path)
                    .args([&app_arg, "--window-size=1280,820"])
                    .spawn()
                    .is_ok()
                {
                    opened_in_app_mode = true;
                    break;
                }
            }
        }

        if !opened_in_app_mode {
            let _ = std::process::Command::new("cmd")
                .args(["/c", "start", url])
                .spawn();
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt::try_init();

    println!("===============================================================");
    println!("       Antares ThreadClip Studio v3.0.0 (Production Release)   ");
    println!("===============================================================");

    let state = Arc::new(AppState::in_memory());
    let info = AppController::get_info();
    println!(
        "[Studio] Hệ điều hành: {} | Phiên bản: {}",
        info.os, info.version
    );

    // Pre-load sample data if available
    let sample_file = PathBuf::from("testdata/test_user_thread_Df5D.json");
    if sample_file.exists() {
        if let Ok(content) = std::fs::read_to_string(&sample_file) {
            if let Ok((p, r)) = AppController::import_offline_json(&state, &content) {
                println!(
                    "[Studio] Đã nạp bài viết Threads mẫu (Df5D-Hak-d3): {} bài, {} phản hồi",
                    p, r
                );
            }
        }
    }

    let ui_dir = find_ui_dir();
    println!("[Studio] Thư mục giao diện (UI): {}", ui_dir.display());

    let server = StudioServer::new(state.clone(), ui_dir);
    let port = match server.start(3000).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[LỖI] Không thể khởi tạo máy chủ HTTP: {}", e);
            eprintln!("Nhấn Enter để thoát...");
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line);
            return Err(e.into());
        }
    };

    let url = format!("http://127.0.0.1:{}", port);
    println!("[Studio] Web UI khả dụng tại: {}", url);
    println!("[Studio] Đang mở trình duyệt mặc định...");
    open_browser(&url);

    println!("---------------------------------------------------------------");
    println!(" Máy chủ Studio ĐANG HOẠT ĐỘNG! Giữ cửa sổ này mở khi sử dụng.");
    println!(" Nhấn Ctrl+C trong cửa sổ này để dừng Studio.");
    println!("---------------------------------------------------------------");

    tokio::signal::ctrl_c().await?;
    println!("\n[Studio] Đang dừng máy chủ Antares ThreadClip Studio...");
    Ok(())
}
