use futures::stream::StreamExt;
use eventsource_stream::Eventsource;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub id: String,
    pub event_type: String,
    pub title: String,
    pub message: String,
    pub timestamp: String,
}

pub struct SSEClient {
    url: String,
    app_handle: AppHandle,
    client: Client,
}

impl SSEClient {
    pub fn new(url: impl Into<String>, app_handle: AppHandle) -> Self {
        Self {
            url: url.into(),
            app_handle,
            client: Client::new(),
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        loop {
            println!("Connecting to SSE at {}", self.url);

            match self.client.get(&self.url).send().await {
                Ok(response) => {
                    let mut stream = response.bytes_stream().eventsource();

                    while let Some(event) = stream.next().await {
                        match event {
                            Ok(event) => {
                                if event.event == "notification" {
                                    if let Ok(notif) = serde_json::from_str::<NotificationEvent>(&event.data) {
                                        // Emit to frontend
                                        self.app_handle.emit_all("new-notification", &notif).unwrap();

                                        // Show notification popup
                                        if let Err(e) = self.show_notification_window(&notif).await {
                                            eprintln!("Failed to show notification: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("SSE stream error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to connect to SSE: {}", e);
                }
            }

            // Reconnect after delay
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    async fn show_notification_window(&self, event: &NotificationEvent) -> anyhow::Result<()> {
        // Get tray position
        let position = self.get_tray_position()?;

        // Create or get notification window
        let window = match self.app_handle.get_window("notification") {
            Some(w) => w,
            None => {
                tauri::WindowBuilder::new(
                    &self.app_handle,
                    "notification",
                    tauri::WindowUrl::App("index.html".into())
                )
                .title("Notification")
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .inner_size(850.0, 450.0)
                .position(position.0, position.1)
                .build()?
            }
        };

        // Send event data to window
        window.emit("display-notification", event)?;
        window.show()?;
        window.set_focus()?;

        // Auto-hide after 5 seconds
        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            window_clone.hide().ok();
        });

        Ok(())
    }

    fn get_tray_position(&self) -> anyhow::Result<(f64, f64)> {
        // Calculate bottom-right position
        // Notification window size
        let window_width = 650.0;
        let window_height = 250.0;
        let margin = 10.0;
        let taskbar_margin = 10.0;

        // Try to get screen size from available monitors
        // For now, use common screen resolutions with fallback
        #[cfg(target_os = "windows")]
        {
            // Windows: bottom-right with taskbar at bottom
            // Assuming 1920x1080 or similar
            let screen_width = 1920.0;
            let screen_height = 1080.0;
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
        #[cfg(target_os = "macos")]
        {
            // macOS: bottom-right (menu bar at top)
            let screen_width = 1920.0;
            let screen_height = 1080.0;
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
        #[cfg(target_os = "linux")]
        {
            // Linux: bottom-right (panel/taskbar varies)
            // Assuming 1920x1080 as most common resolution
            let screen_width = 1920.0;
            let screen_height = 1080.0;
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
    }
}
