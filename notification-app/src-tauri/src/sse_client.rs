use futures::stream::StreamExt;
use eventsource_stream::Eventsource;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Emitter};

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
                                        self.app_handle.emit("new-notification", &notif).unwrap();

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
        // Notification window dimensions - sized for notification content
        const WINDOW_WIDTH: f64 = 520.0;
        const WINDOW_HEIGHT: f64 = 180.0;

        // Get tray position
        let position = self.get_tray_position(WINDOW_WIDTH, WINDOW_HEIGHT)?;

        // Create or get notification window
        let window = match self.app_handle.get_webview_window("notification") {
            Some(w) => {
                // Update position for existing window
                w.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: position.0 as i32,
                    y: position.1 as i32,
                }))?;
                w
            },
            None => {
                tauri::WebviewWindowBuilder::new(
                    &self.app_handle,
                    "notification",
                    tauri::WebviewUrl::App("index.html".into())
                )
                .title("Notification")
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
                .position(position.0, position.1)
                .build()?
            }
        };

        // Send event data to window
        window.emit("display-notification", event)?;
        window.show()?;
        window.set_focus()?;

        // Auto-hide after 10 seconds
        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            window_clone.hide().ok();
        });

        Ok(())
    }

    fn get_tray_position(&self, window_width: f64, window_height: f64) -> anyhow::Result<(f64, f64)> {
        // Margins from screen edges
        let margin = 20.0;
        let taskbar_margin = 40.0; // Extra margin for taskbar on Windows

        // Get the primary monitor's dimensions
        let monitor = self.app_handle
            .primary_monitor()
            .map_err(|e| anyhow::anyhow!("Failed to get primary monitor: {}", e))?
            .ok_or_else(|| anyhow::anyhow!("No primary monitor found"))?;

        let screen_size = monitor.size();
        let screen_width = screen_size.width as f64;
        let screen_height = screen_size.height as f64;

        // Calculate bottom-right position with appropriate margins
        #[cfg(target_os = "windows")]
        {
            // Windows: account for taskbar at bottom
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
        #[cfg(target_os = "macos")]
        {
            // macOS: menu bar is at top, dock can be at bottom/sides
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
        #[cfg(target_os = "linux")]
        {
            // Linux: panel/taskbar position varies by DE
            let x = screen_width - window_width - margin;
            let y = screen_height - window_height - margin - taskbar_margin;
            Ok((x, y))
        }
    }
}
