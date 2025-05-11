use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use tokio::{net::TcpListener, sync::RwLock};

mod tle;
mod orbit;
mod api;

#[derive(Clone)]
pub struct AppState {
    pub tle: Arc<RwLock<tle::TleStore>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // preload cache
    let cache = tle::TleStore::new().await?;
    let state = AppState {
        tle: Arc::new(RwLock::new(cache)),
    };

    // ── build routes first ──
    let app = Router::new()
        .route("/", get(|| async { "Satellite Tracker API Online" }))
        .route("/api/satellite/:id", get(api::satellite))
        // ── then satisfy the state requirement ──
        .with_state(state);

    // ── serve ──
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;  // now compiles
    Ok(())
}
