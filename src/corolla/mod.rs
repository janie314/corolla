use crate::corolla::db::DB;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;

mod db;
mod error;

pub type Args = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query_endpoint(
    Path(query_name): Path<String>,
    Query(params): Query<Args>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.read_query(&query_name, &params).await
}

#[axum::debug_handler]
async fn write_query_endpoint(
    Path(query_name): Path<String>,
    Query(params): Query<Args>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.write_query(&query_name, &params).await
}

pub async fn serve(
    route_base: &str,
    db_path: &str,
    port: i64,
    init_statements: &[&str],
    queries: &[(&str, &str, Vec<String>)],
) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new(db_path, init_statements, queries)
        .await
        .expect("oh no");
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route(
            &format!("{route_base}/read/:query_name"),
            get(read_query_endpoint),
        )
        .route(
            &format!("{route_base}/write/:query_name"),
            post(write_query_endpoint),
        )
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
