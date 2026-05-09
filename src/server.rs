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

use aoike_core::FileStats;

#[derive(RustEmbed)]
#[folder = "frontend/"]
struct Assets;

#[derive(Serialize)]
struct StatsResponse {
    counts: std::collections::HashMap<String, usize>,
    total: usize,
}

pub fn create_app(stats: FileStats) -> Router {
    Router::new()
        .route("/api/stats", get(get_stats))
        .route("/", get(index_handler))
        .route("/{*path}", get(static_handler))
        .with_state(Arc::new(stats))
}

async fn get_stats(State(stats): State<Arc<FileStats>>) -> impl IntoResponse {
    let (counts, total) = stats.get_stats();
    Json(StatsResponse { counts, total })
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
) -> anyhow::Result<()> {
    let app = create_app(stats);
    
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    tracing::info!("HTTP server listening on http://{}", bind_addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
