use self::{
    error::Error,
    spec::{read_spec, Spec},
};
use crate::corolla::db::DB;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use log::info;
use std::collections::HashMap;
use tower_http::services::ServeDir;

mod db;
mod error;
mod spec;
mod version;

pub type Args = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query_endpoint(
    Path(query): Path<String>,
    Query(params): Query<Args>,
    State(db): State<DB>,
) -> impl IntoResponse {
    match db.read_query(&query, &params, None).await {
        Ok(res) => Json(res).into_response(),
        Err(e) => e.into_response(),
    }
}

#[axum::debug_handler]
async fn write_query_endpoint(
    Path(query): Path<String>,
    State(db): State<DB>,
    Json(params): Json<Args>,
) -> impl IntoResponse {
    db.write_query(&query, &params, None).await
}
/// Internal core method that runs the Corolla server.
///
/// Arguments:
///
/// * `route_base` - The base HTTP route. For instance, if `route_base == "/api"` then the `/read/:query` endpoint will be served under `/api/read/:query`.
/// * `db_path` - Filepath to the SQLite database.
/// * `static_path` - Filepath to static file directory.
/// * `port` - The port the server will listen on.
/// * `init_statements` - A list of SQL statements that will be executed to initialize the database, in order.
/// * `queries` - A lookup table of SQL queries.
async fn serve(
    route_base: &str,
    port: i64,
    db_path: &str,
    static_path: &str,
    spec: &Spec,
) -> Result<(), Error> {
    let addr = format!("0.0.0.0:{}", port);
    let conn = DB::from_spec(db_path, &spec).await?;
    info!("listening on {}", &addr);
    let app = Router::new()
        .route(
            &format!("{route_base}/read/:query"),
            get(read_query_endpoint),
        )
        .route(
            &format!("{route_base}/write/:query"),
            post(write_query_endpoint),
        )
        .nest_service(&format!("{route_base}/static"), ServeDir::new(static_path))
        .with_state(conn);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|_| Error::Server)?;
    Ok(())
}
/// Run a Corolla web server according to server config and spec.json
///
/// Arguments:
///
/// * `route_base` - The base HTTP route. For instance, if `route_base == "/api"` then the `/read/:query` endpoint will be served under `/api/read/:query`.
/// * `port` - The port the server will listen on.
/// * `db_path` - Filepath to the SQLite database.
/// * `static_path` - Filepath to static file directory.
/// * `spec_path` - Filepath to the spec.json.
pub async fn run(
    route_base: &str,
    port: i64,
    db_path: &str,
    static_path: &str,
    spec_path: &str,
) -> Result<(), Error> {
    let spec = read_spec(&spec_path)?;
    serve(route_base, port, db_path, static_path, &spec).await?;
    Ok(())
}
