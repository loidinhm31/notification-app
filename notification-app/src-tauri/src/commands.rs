use tauri::{Emitter, State};
use crate::{AppState, NotificationEvent};

#[tauri::command]
pub async fn start_sse_connection(_state: State<'_, AppState>) -> Result<(), String> {
    println!("Starting SSE connection...");
    Ok(())
}

#[tauri::command]
pub async fn stop_sse_connection(_state: State<'_, AppState>) -> Result<(), String> {
    println!("Stopping SSE connection...");
    Ok(())
}

#[tauri::command]
pub async fn show_notification(
    app_handle: tauri::AppHandle,
    event: NotificationEvent
) -> Result<(), String> {
    app_handle.emit("new-notification", &event)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn show_animation(
    app_handle: tauri::AppHandle,
    animation_url: String
) -> Result<(), String> {
    // Create animation window
    let window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "animation",
        tauri::WebviewUrl::App(animation_url.into())
    )
    .title("Animation")
    .fullscreen(false)
    .inner_size(800.0, 600.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn check_server_health() -> Result<bool, String> {
    match reqwest::get("http://localhost:9201/health").await {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false)
    }
}
