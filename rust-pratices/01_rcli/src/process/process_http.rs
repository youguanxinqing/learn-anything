use axum::{
    extract::{Path, State}, http::StatusCode, routing::get, serve, Router
};
use tower_http::services::ServeDir;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;

use crate::cli::http::HttpSubCommand;

pub async fn process_http(cmd: HttpSubCommand) -> anyhow::Result<()> {
    match cmd {
        HttpSubCommand::Serve(opts) => process_serve(opts.dir, opts.port).await?,
    };

    Ok(())
}

struct AppState {
    dir: PathBuf,
}

async fn index(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = state.dir.join(&path);
    let p = std::path::Path::new(&file_path);
    if !p.exists() {
        return (StatusCode::NOT_FOUND, format!("Not found '{}'", path));
    }

    match tokio::fs::read(&file_path).await {
        Ok(content) => (
            StatusCode::OK,
            String::from_utf8_lossy(&content).to_string(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to open '{}', err: {}", path, e.to_string()),
        ),
    }
}

async fn process_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    info!(
        "Serving at http://0.0.0.0:{} under '{}'",
        port,
        dir.to_str().unwrap()
    );

    let serve_dir = ServeDir::new(".");

    let app = Router::new()
        .route("/*path", get(index))
        .nest_service("/tower", serve_dir)
        .with_state(Arc::new(AppState { dir }));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
