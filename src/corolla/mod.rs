use self::{error::Error, spec::read_spec};
use crate::corolla::db::DB;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::collections::HashMap;

mod db;
mod error;
mod spec;

pub type Args = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query_endpoint(
    Path(query): Path<String>,
    Query(params): Query<Args>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.read_query(&query, &params).await
}

#[axum::debug_handler]
async fn write_query_endpoint(
    Path(query): Path<String>,
    State(db): State<DB>,
    Json(params): Json<Args>,
) -> impl IntoResponse {
    db.write_query(&query, &params).await
}
/// Internal core method that runs the Corolla server.
///
/// # Arguments
///
/// * `route_base` - The base HTTP route. For instance, if `route_base == "/api"` then the `/read/:query` endpoint will be served under `/api/read/:query`.
/// * `db_path` - Filepath to the SQLite database.
/// * `port` - The port the server will listen on.
/// * `init_statements` - A list of SQL statements that will be executed to initialize the database, in order.
/// * `queries` - A lookup table of SQL queries.
async fn serve(
    route_base: &str,
    port: i64,
    db_path: &str,
    init_statements: &Vec<String>,
    queries: &HashMap<String, spec::Query>,
) -> Result<(), Error> {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .map_err(|_| Error::BadPort)?;
    let conn = DB::new(db_path, init_statements, queries).await?;
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route(
            &format!("{route_base}/read/:query"),
            get(read_query_endpoint),
        )
        .route(
            &format!("{route_base}/write/:query"),
            post(write_query_endpoint),
        )
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| Error::Server)?;
    Ok(())
}
/// Run a Corolla web server according to server config and spec.json
///
/// # Arguments
///
/// * `route_base` - The base HTTP route. For instance, if `route_base == "/api"` then the `/read/:query` endpoint will be served under `/api/read/:query`.
/// * `port` - The port the server will listen on.
/// * `db_path` - Filepath to the SQLite database.
/// * `spec_path` - Filepath to the spec.json.
pub async fn run(route_base: &str, port: i64, db_path: &str, spec_path: &str) -> Result<(), Error> {
    let spec = read_spec(&spec_path)?;
    serve(route_base, port, db_path, &spec.init, &spec.queries).await?;
    Ok(())
}
