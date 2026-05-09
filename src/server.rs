use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::Serialize;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use aoike_core::{FileStats, TaskIndex};

#[derive(RustEmbed)]
#[folder = "frontend/"]
struct Assets;

#[derive(Serialize)]
struct StatsResponse {
    counts: std::collections::HashMap<String, usize>,
    total: usize,
}

#[derive(Clone)]
struct AppState {
    stats: FileStats,
    task_index: TaskIndex,
}

pub fn create_app(stats: FileStats, task_index: TaskIndex) -> Router {
    let state = AppState {
        stats,
        task_index,
    };

    Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/events", get(stats_sse_handler))
        .route("/api/tasks", get(get_tasks))
        .route("/api/task-events", get(tasks_sse_handler))
        .route("/", get(index_handler))
        .route("/{*path}", get(static_handler))
        .with_state(Arc::new(state))
}

async fn get_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (counts, total) = state.stats.get_stats();
    Json(StatsResponse { counts, total })
}

async fn stats_sse_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rx = state.stats.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|result| {
            match result {
                Ok(update) => {
                    let json = serde_json::to_string(&update).ok()?;
                    Some(Ok::<_, std::convert::Infallible>(
                        format!("data: {}\n\n", json)
                    ))
                }
                Err(_) => None,
            }
        });

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header("X-Accel-Buffering", "no")
        .body(Body::from_stream(stream))
        .unwrap()
}

async fn get_tasks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tasks = state.task_index.get_all_tasks();
    let (total_todo, total_done) = state.task_index.get_stats();
    
    Json(serde_json::json!({
        "tasks": tasks,
        "total_todo": total_todo,
        "total_done": total_done,
    }))
}

async fn tasks_sse_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rx = state.task_index.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|result| {
            match result {
                Ok(update) => {
                    let json = serde_json::to_string(&update).ok()?;
                    Some(Ok::<_, std::convert::Infallible>(
                        format!("data: {}\n\n", json)
                    ))
                }
                Err(_) => None,
            }
        });

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header("X-Accel-Buffering", "no")
        .body(Body::from_stream(stream))
        .unwrap()
}

async fn index_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("/index.html")).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    
    // If path is empty, serve index.html
    let path = if path.is_empty() { "index.html" } else { path };
    
            match Assets::get(path) {
                Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, mime.as_ref())
                        .body(Body::from(content.data))
                        .unwrap()
                }
                None => {
                    // Try to serve index.html for SPA routing
                    match Assets::get("index.html") {
                        Some(content) => Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "text/html")
                            .body(Body::from(content.data))
                            .unwrap(),
                        None => Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("Not Found"))
                            .unwrap(),
                    }
                }
            }
}

pub async fn run_server(
    bind_addr: &str,
    stats: FileStats,
    task_index: TaskIndex,
) -> anyhow::Result<()> {
    let app = create_app(stats, task_index);
    
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    tracing::info!("HTTP server listening on http://{}", bind_addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
