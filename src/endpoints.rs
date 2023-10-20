use crate::db::DB;
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::collections::HashMap;

/*

WAIT YOU DON'T NEED ANY OF THIS GET/JSON NONSENSE
JUST USE THE QUERY ARGS IN YOUR GET QUERY PARAMETER

*/
pub type Arg = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query(
    Path(query_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(db): State<DB>,
) -> Json<String> {
    let x = params.get("joe").unwrap_or(&String::default()).clone();
    Json(x)
}

pub async fn serve(route_base: &str, db_path: &str, port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new(db_path).await.expect("oh no");
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route(&format!("{route_base}/read/:query_name"), get(read_query))
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
