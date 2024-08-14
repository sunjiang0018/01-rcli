use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::routing::get;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", &path, addr);

    let state = HttpServeState { path };

    let router = axum::Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await?;
    anyhow::Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", file);

    if !file.exists() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else {
        match tokio::fs::read_to_string(file).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        }
    }
}
