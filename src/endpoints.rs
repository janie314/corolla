use crate::db::DB;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::collections::HashMap;

pub type Arg = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query_endpoint(
    Path(query_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(db): State<DB>,
) -> Json<String> {
    match db.read_query(&query_name, &params).await {
        Ok(_) => Json("good".to_string()),
        Err(_) => Json("bad".to_string()),
    }
}

pub async fn serve(route_base: &str, db_path: &str, port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new(db_path, &["create table if not exists t (c text);"])
        .await
        .expect("oh no");
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route(
            &format!("{route_base}/read/:query_name"),
            get(read_query_endpoint),
        )
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
