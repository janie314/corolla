use axum::Json;
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Error, Pool, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

pub type QueryArgs = HashMap<String, String>;

#[derive(Clone)]
pub struct Query {
    sql_template: String,
    args: QueryArgs,
}

#[derive(Clone)]
pub struct DB {
    conn: Arc<RwLock<Pool<Sqlite>>>,
}

impl DB {
    pub async fn new(filepath: &str) -> Result<Self, Error> {
        let conn = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(filepath)
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await?;
        query("create table if not exists t (c int);")
            .execute(&conn)
            .await?;
        let db = DB {
            conn: Arc::new(RwLock::new(conn)),
        };
        Ok(db)
    }
    pub async fn write_query(&self, query_name: &str, args: Json<QueryArgs>) -> Result<(), Error> {
        let c = self.conn.write().await;
        query("insert into t values (?);")
            .bind("wal")
            .execute(c.deref())
            .await?;
        Ok(())
    }
}
