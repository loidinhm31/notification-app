use tauri::WebviewWindow;
use std::collections::HashMap;

pub struct WindowManager {
    windows: HashMap<String, WebviewWindow>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn register_window(&mut self, id: String, window: WebviewWindow) {
        self.windows.insert(id, window);
    }

    pub fn get_window(&self, id: &str) -> Option<&WebviewWindow> {
        self.windows.get(id)
    }

    pub fn close_all(&mut self) {
        for (_, window) in self.windows.drain() {
            window.close().ok();
        }
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}
