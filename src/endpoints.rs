use crate::db::DB;
use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[axum::debug_handler]
async fn sumbot(Path(num): Path<i64>, State(state): State<DB>) -> Json<i64> {
    let vol = 30 + num;
    Json(vol)
}

pub async fn serve(port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new("/tmp/a/sqlite3").await.expect("oh no");
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route("/sumbot/:num", post(sumbot))
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
