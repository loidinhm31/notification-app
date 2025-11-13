// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    Manager
};
use std::sync::Arc;
use parking_lot::Mutex;

mod commands;
mod sse_client;
mod window_manager;

use commands::*;
use sse_client::SSEClient;
use window_manager::WindowManager;

pub use sse_client::NotificationEvent;

#[derive(Default, Clone)]
pub struct AppState {
    sse_client: Arc<Mutex<Option<SSEClient>>>,
    window_manager: Arc<Mutex<WindowManager>>,
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show_notif = CustomMenuItem::new("show_notif".to_string(), "Test Notification");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show_notif)
        .add_item(settings)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(AppState::default())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show_notif" => {
                    // Create a test notification
                    let test_notification = NotificationEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        event_type: "test".to_string(),
                        title: "Test Notification".to_string(),
                        message: "This is a test notification to verify everything is working correctly. Take a moment to breathe and relax.".to_string(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    };

                    // Emit the notification event to the frontend
                    if let Err(e) = app.emit_all("new-notification", &test_notification) {
                        eprintln!("Failed to emit test notification: {}", e);
                    }
                }
                "settings" => {
                    // Open settings window
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            start_sse_connection,
            stop_sse_connection,
            show_notification,
            show_animation,
            check_server_health
        ])
        .setup(|app| {
            // Initialize SSE client
            let app_handle = app.app_handle();
            let state: tauri::State<AppState> = app.state();
            let state_clone = state.inner().clone();

            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let mut client = SSEClient::new(
                    "http://localhost:9201/api/events/stream",
                    app_handle_clone
                );

                if let Err(e) = client.connect().await {
                    eprintln!("Failed to connect to SSE: {}", e);
                }

                *state_clone.sse_client.lock() = Some(client);
            });

            // Start health check
            let app_handle_health = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    // Check notification-server health
                    match reqwest::get("http://localhost:9201/health").await {
                        Ok(resp) if resp.status().is_success() => {
                            println!("Server healthy");
                        }
                        _ => {
                            eprintln!("Server unhealthy, falling back to polling");
                            let _ = app_handle_health.emit_all("notification-server-down", ());
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
