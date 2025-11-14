// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager, Emitter,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
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
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_sse_connection,
            stop_sse_connection,
            show_notification,
            show_animation,
            check_server_health
        ])
        .setup(|app| {
            // Create the tray menu
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_notif_i = MenuItem::with_id(app, "show_notif", "Test Notification", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[
                &show_notif_i,
                &settings_i,
                &quit_i,
            ])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
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
                        if let Err(e) = app.emit("new-notification", &test_notification) {
                            eprintln!("Failed to emit test notification: {}", e);
                        }
                    }
                    "settings" => {
                        // Open settings window
                    }
                    _ => {}
                })
                .build(app)?;

            // Initialize SSE client
            let app_handle = app.handle().clone();
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
                            let _ = app_handle_health.emit("notification-server-down", ());
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
