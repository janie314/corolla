use crate::corolla::db::DB;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::collections::HashMap;

use self::error::Error;

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
    State(db): State<DB>,
    Json(params): Json<Args>,
) -> impl IntoResponse {
    db.write_query(&query_name, &params).await
}

pub async fn serve(
    route_base: &str,
    db_path: &str,
    port: i64,
    init_statements: &[&str],
    queries: &[(&str, &str, Vec<String>)],
) -> Result<(), Error> {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .map_err(|_| Error::BadPort)?;
    let conn = DB::new(db_path, init_statements, queries).await?;
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
        .map_err(|_| Error::Server)?;
    Ok(())
}
