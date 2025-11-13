use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub id: String,
    pub event_type: String,
    pub title: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl NotificationEvent {
    pub fn new(
        event_type: String,
        title: String,
        message: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            title,
            message,
            timestamp: Utc::now(),
        }
    }

    pub fn new_with_timestamp(
        event_type: String,
        title: String,
        message: String,
        timestamp: Option<String>,
    ) -> Self {
        let parsed_timestamp = timestamp
            .and_then(|ts| DateTime::parse_from_rfc3339(&ts).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        Self {
            id: Uuid::new_v4().to_string(),
            event_type,
            title,
            message,
            timestamp: parsed_timestamp,
        }
    }
}
