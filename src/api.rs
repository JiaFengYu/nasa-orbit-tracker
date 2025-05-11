use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

use crate::{orbit, AppState};

pub async fn satellite(
    Path(id): Path<u32>,
    State(app): State<AppState>,
) -> Json<serde_json::Value> {
    {
        let mut store = app.tle.write().await;
        let _ = store.refresh_if_stale().await;
    }

    let store = app.tle.read().await;
    match store.get(id) {
        Some(el) => match orbit::propagate(&el) {   // ← pass struct ref
            Ok(pts) => Json(json!(pts)),
            Err(e)  => Json(json!({ "error": e.to_string() })),
        },
        None => Json(json!({ "error": "satellite not found" })),
    }
}
