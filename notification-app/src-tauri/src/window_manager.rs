use tauri::Window;
use std::collections::HashMap;

pub struct WindowManager {
    windows: HashMap<String, Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn register_window(&mut self, id: String, window: Window) {
        self.windows.insert(id, window);
    }

    pub fn get_window(&self, id: &str) -> Option<&Window> {
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
