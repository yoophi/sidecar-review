mod application;
mod domain;
mod adapters {
    pub mod mcp_http;
    pub mod file_repo;
    pub mod tauri_ui;
}

use std::sync::Arc;

use tauri::Manager;

use adapters::{mcp_http, file_repo::FileRepo, tauri_ui};
use application::ReviewService;

const ADDR: &str = "127.0.0.1:7437";

// composition root: wires the adapters to the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let service = Arc::new(ReviewService::new(
                Box::new(FileRepo::open(dir.join("documents.json"))?),
                Box::new(tauri_ui::TauriNotifier(app.handle().clone())),
            ));
            app.manage(service.clone());

            // bind here so a busy port fails the launch loudly instead of leaving a dead MCP endpoint
            let listener = std::net::TcpListener::bind(ADDR)?;
            listener.set_nonblocking(true)?;
            tauri::async_runtime::spawn(async move {
                let listener = tokio::net::TcpListener::from_std(listener).unwrap();
                axum::serve(listener, mcp_http::router(service)).await.unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![tauri_ui::documents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
