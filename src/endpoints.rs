use crate::db::DB;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::collections::HashMap;

pub type Args = HashMap<String, String>;

#[axum::debug_handler]
async fn read_query_endpoint(
    Path(query_name): Path<String>,
    Query(params): Query<Args>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.read_query(&query_name, &params).await
}

pub async fn serve(route_base: &str, db_path: &str, port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new(
        db_path,
        &["create table if not exists t (c text);"],
        &[(
            "q1",
            "select c from t where c = ?;",
            Vec::from(["val".to_string()]),
        )],
    )
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
