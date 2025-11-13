use axum::{
    extract::State,
    http::StatusCode,
    response::{sse::Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::{Stream, StreamExt};
use serde::Deserialize;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::{Any, CorsLayer};

mod models;
mod services;

use models::NotificationEvent;
use services::EventBroadcaster;

#[derive(Clone)]
struct AppState {
    broadcaster: Arc<EventBroadcaster>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let broadcaster = Arc::new(EventBroadcaster::new());
    let state = AppState { broadcaster };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/events/stream", get(sse_handler))
        .route("/api/events/poll", get(poll_events))
        .route("/api/events", post(create_event))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9201")
        .await
        .unwrap();

    println!("Server running on http://localhost:9201");

    axum::serve(listener, app).await.unwrap();
}

// SSE endpoint
async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.broadcaster.subscribe();
    let stream = BroadcastStream::new(rx).map(|result| {
        match result {
            Ok(event) => {
                let json = serde_json::to_string(&event).unwrap();
                Ok(Event::default()
                    .event("notification")
                    .data(json))
            }
            Err(_) => {
                Ok(Event::default()
                    .event("error")
                    .data("Stream error"))
            }
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

// Polling endpoint (fallback)
async fn poll_events(
    State(_state): State<AppState>,
) -> Json<Vec<NotificationEvent>> {
    // Return recent events (in a real implementation, you'd store these)
    Json(vec![])
}

// Create event endpoint
async fn create_event(
    State(state): State<AppState>,
    Json(payload): Json<CreateEventRequest>,
) -> (StatusCode, Json<NotificationEvent>) {
    let event = NotificationEvent::new_with_timestamp(
        payload.event_type,
        payload.title,
        payload.message,
        payload.timestamp,
    );

    state.broadcaster.send(event.clone());

    (StatusCode::CREATED, Json(event))
}

// Health check endpoint
async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

#[derive(Debug, Deserialize)]
struct CreateEventRequest {
    #[serde(default = "default_event_type")]
    event_type: String,
    title: String,
    message: String,
    #[serde(default)]
    timestamp: Option<String>,
}

fn default_event_type() -> String {
    "notification".to_string()
}
