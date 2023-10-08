use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::Response,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tokio::sync::RwLock;

#[derive(Clone)]
struct ServerState {
    mu: Arc<RwLock<()>>,
}

#[axum::debug_handler]
async fn get_vol(Path((num)): Path<(i64)>, State(_state): State<ServerState>) -> Json<i64> {
    let vol = 30 + num;
    Json(vol)
}

pub async fn serve(port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let state = ServerState {
        mu: Arc::new(RwLock::new(())),
    };
    println!("trying to listen on {}", &addr);
    let app = Router::new().route("/vol", get(get_vol)).with_state(state);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
