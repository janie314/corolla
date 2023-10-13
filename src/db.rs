use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Error, Pool, Sqlite, SqlitePool,
};
use std::{ops::Deref, sync::Arc};
use tokio::sync::RwLock;

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
    pub async fn write_query(&self, num: i64) -> Result<(), Error> {
        let c = self.conn.write().await;
        query("insert into t values (?);")
            .bind(num)
            .execute(c.deref())
            .await?;
        Ok(())
    }
}
