use crate::db::DB;
use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};



#[axum::debug_handler]
async fn sumbot(Path(num): Path<i64>, State(db): State<DB>) -> Json<i64> {
    let vol = 30 + num;
    db.write_query(num).await.expect("bad");
    Json(vol)
}

pub async fn serve(filepath: &str, port: i64) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("i could not listen on the port");
    let conn = DB::new(filepath).await.expect("oh no");
    println!("trying to listen on {}", &addr);
    let app = Router::new()
        .route("/sumbot/:num", post(sumbot))
        .with_state(conn);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("the server crashed");
}
