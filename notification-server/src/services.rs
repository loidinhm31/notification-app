use crate::models::NotificationEvent;
use tokio::sync::broadcast;

pub struct EventBroadcaster {
    tx: broadcast::Sender<NotificationEvent>,
}

impl EventBroadcaster {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn send(&self, event: NotificationEvent) {
        if let Err(e) = self.tx.send(event) {
            eprintln!("Failed to broadcast event: {}", e);
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<NotificationEvent> {
        self.tx.subscribe()
    }
}
