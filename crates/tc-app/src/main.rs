use tc_app::{AppController, AppState};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt::try_init();
    println!("Starting Antares ThreadClip Studio v3.0.0...");
    let _state = AppState::in_memory();
    let info = AppController::get_info();
    println!("Studio initialized on {}. App: {}", info.os, info.name);
}
