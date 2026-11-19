use crate::commands::AppController;
use crate::state::AppState;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

pub struct StudioServer {
    state: Arc<AppState>,
    ui_dir: PathBuf,
}

impl StudioServer {
    pub fn new(state: Arc<AppState>, ui_dir: PathBuf) -> Self {
        Self { state, ui_dir }
    }

    pub async fn start(&self, preferred_port: u16) -> Result<u16, std::io::Error> {
        let mut port = preferred_port;
        let listener = loop {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            match TcpListener::bind(addr).await {
                Ok(l) => break l,
                Err(e) if port < preferred_port + 50 => {
                    port += 1;
                    if port == preferred_port + 50 {
                        return Err(e);
                    }
                }
                Err(e) => return Err(e),
            }
        };

        info!(
            "Antares ThreadClip Studio HTTP server listening on http://127.0.0.1:{}",
            port
        );

        let state = self.state.clone();
        let ui_dir = self.ui_dir.clone();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((socket, _)) => {
                        let state = state.clone();
                        let ui_dir = ui_dir.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(socket, state, ui_dir).await {
                                warn!("Connection notice: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept incoming connection: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(port)
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    state: Arc<AppState>,
    ui_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = [0u8; 16384];
    let n = socket.read(&mut buffer).await?;
    if n == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buffer[..n]);
    let mut lines = request_str.lines();
    let request_line = match lines.next() {
        Some(line) => line,
        None => return Ok(()),
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let raw_path = parts.next().unwrap_or("/");

    if method == "OPTIONS" {
        let response = "HTTP/1.1 204 No Content\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
Connection: close\r\n\r\n";
        socket.write_all(response.as_bytes()).await?;
        return Ok(());
    }

    let path = raw_path.split('?').next().unwrap_or("/");

    if path.starts_with("/api/") {
        handle_api_route(&mut socket, method, path, &request_str, &state).await?;
        return Ok(());
    }

    handle_static_file(&mut socket, path, &ui_dir).await?;
    Ok(())
}

fn get_mime_type(path: &str) -> &'static str {
    if path.ends_with(".html") || path.ends_with(".htm") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else if path.ends_with(".webp") {
        "image/webp"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".ttf") {
        "font/ttf"
    } else if path.ends_with(".mp4") {
        "video/mp4"
    } else if path.ends_with(".webm") {
        "video/webm"
    } else if path.ends_with(".wav") {
        "audio/wav"
    } else if path.ends_with(".mp3") {
        "audio/mpeg"
    } else {
        "application/octet-stream"
    }
}

async fn send_response(
    socket: &mut TcpStream,
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), std::io::Error> {
    let headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );
    socket.write_all(headers.as_bytes()).await?;
    socket.write_all(body).await?;
    socket.flush().await?;
    Ok(())
}

async fn send_json(
    socket: &mut TcpStream,
    status_code: u16,
    json: &str,
) -> Result<(), std::io::Error> {
    send_response(
        socket,
        status_code,
        if status_code == 200 { "OK" } else { "Error" },
        "application/json; charset=utf-8",
        json.as_bytes(),
    )
    .await
}

async fn handle_api_route(
    socket: &mut TcpStream,
    method: &str,
    path: &str,
    request_str: &str,
    state: &AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match (method, path) {
        ("GET", "/api/info") => {
            let info = AppController::get_info();
            let json = serde_json::to_string(&info)?;
            send_json(socket, 200, &json).await?;
        }
        ("GET", "/api/health") => {
            send_json(socket, 200, r#"{"status":"healthy","version":"3.0.0"}"#).await?;
        }
        ("GET", "/api/posts") => {
            let posts = AppController::list_posts(state, 50).unwrap_or_default();
            let json = serde_json::to_string(&posts)?;
            send_json(socket, 200, &json).await?;
        }
        ("POST", "/api/import") => {
            if let Some(pos) = request_str.find("\r\n\r\n") {
                let body = &request_str[pos + 4..];
                match AppController::import_offline_json(state, body) {
                    Ok((p, r)) => {
                        let json = format!(r#"{{"posts":{},"replies":{}}}"#, p, r);
                        send_json(socket, 200, &json).await?;
                    }
                    Err(e) => {
                        let err = format!(r#"{{"error":"{}"}}"#, e);
                        send_json(socket, 400, &err).await?;
                    }
                }
            } else {
                send_json(socket, 400, r#"{"error":"missing_body"}"#).await?;
            }
        }
        _ => {
            if path.starts_with("/api/posts/") && path.ends_with("/tree") {
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() >= 4 {
                    let id = parts[3];
                    match AppController::get_post_tree(state, id) {
                        Ok(tree) => {
                            let json = serde_json::to_string(&tree)?;
                            send_json(socket, 200, &json).await?;
                            return Ok(());
                        }
                        Err(e) => {
                            let err = format!(r#"{{"error":"{}"}}"#, e);
                            send_json(socket, 404, &err).await?;
                            return Ok(());
                        }
                    }
                }
            } else if path.starts_with("/api/posts/") && path.ends_with("/score") {
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() >= 4 {
                    let id = parts[3];
                    match AppController::score_post(state, id) {
                        Ok(score) => {
                            let json = serde_json::to_string(&score)?;
                            send_json(socket, 200, &json).await?;
                            return Ok(());
                        }
                        Err(e) => {
                            let err = format!(r#"{{"error":"{}"}}"#, e);
                            send_json(socket, 404, &err).await?;
                            return Ok(());
                        }
                    }
                }
            }

            send_json(socket, 404, r#"{"error":"endpoint_not_found"}"#).await?;
        }
    }
    Ok(())
}

async fn handle_static_file(
    socket: &mut TcpStream,
    path: &str,
    ui_dir: &Path,
) -> Result<(), std::io::Error> {
    let clean_path = path.trim_start_matches('/').replace("..", "");
    let candidate = if clean_path.is_empty() || clean_path == "index.html" {
        ui_dir.join("index.html")
    } else {
        ui_dir.join(&clean_path)
    };

    let target_file = if candidate.is_file() {
        candidate
    } else {
        ui_dir.join("index.html")
    };

    if target_file.is_file() {
        match tokio::fs::read(&target_file).await {
            Ok(bytes) => {
                let mime = get_mime_type(&target_file.to_string_lossy());
                send_response(socket, 200, "OK", mime, &bytes).await?;
            }
            Err(e) => {
                let err_msg = format!("Cannot read file: {}", e);
                send_response(socket, 500, "Error", "text/plain", err_msg.as_bytes()).await?;
            }
        }
    } else {
        let fallback_html = format!(
            r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <title>Antares ThreadClip Studio</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #020617; color: #f8fafc; padding: 2.5rem; }}
        h1 {{ color: #38bdf8; }}
        .box {{ background: #0f172a; border: 1px solid #1e293b; border-radius: 0.5rem; padding: 1.5rem; margin-top: 1rem; }}
        code {{ background: #1e293b; padding: 0.2rem 0.4rem; border-radius: 0.25rem; color: #f43f5e; }}
    </style>
</head>
<body>
    <h1>Antares ThreadClip Studio v3.0.0</h1>
    <div class="box">
        <h3>Server Backend đang hoạt động tại cổng hiện tại!</h3>
        <p>Thư mục UI tĩnh không tìm thấy tại: <code>{}</code></p>
        <p>Vui lòng đảm bảo thư mục <code>ui/</code> được đóng gói cùng file chạy hoặc chạy lệnh <code>npm run build</code> trong thư mục <code>ui/</code>.</p>
    </div>
</body>
</html>"#,
            ui_dir.display()
        );
        send_response(socket, 200, "OK", "text/html; charset=utf-8", fallback_html.as_bytes()).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_studio_server_api_and_static() {
        let state = Arc::new(AppState::in_memory());
        let dir = tempdir().unwrap();
        let index_path = dir.path().join("index.html");
        std::fs::write(&index_path, "<html><body>Hello Studio</body></html>").unwrap();

        let server = StudioServer::new(state, dir.path().to_path_buf());
        let port = server.start(19500).await.unwrap();

        // 1. Test API /api/info
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        stream
            .write_all(b"GET /api/info HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")
            .await
            .unwrap();

        let mut buffer = [0u8; 4096];
        let n = stream.read(&mut buffer).await.unwrap();
        let resp = String::from_utf8_lossy(&buffer[..n]);
        assert!(resp.contains("200 OK"));
        assert!(resp.contains("Antares ThreadClip Studio"));

        // 2. Test static file serving
        let mut stream2 = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
        stream2
            .write_all(b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")
            .await
            .unwrap();

        let mut buffer2 = [0u8; 4096];
        let n2 = stream2.read(&mut buffer2).await.unwrap();
        let resp2 = String::from_utf8_lossy(&buffer2[..n2]);
        assert!(resp2.contains("200 OK"));
        assert!(resp2.contains("Hello Studio"));
    }
}
