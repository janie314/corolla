use std::{ops::Deref, sync::Arc};

use sqlx::{query, Connection, Error, Pool, Sqlite, SqliteConnection, SqlitePool};
use tokio::sync::RwLock;

pub struct DB {
    conn: Arc<RwLock<Pool<Sqlite>>>,
}

impl DB {
    pub async fn new(filepath: String) -> Result<Self, Error> {
        let conn = SqlitePool::connect(&format!("sqlite::memory:{filepath}")).await?;
        let db = DB {
            conn: Arc::new(RwLock::new(conn)),
        };
        Ok(db)
    }
    pub async fn write_query(&self, num: i64) -> Result<(), Error> {
        let c = self.conn.write().await;
        query("insert into t values (1);")
            .execute(c.deref())
            .await?;
        Ok(())
    }
}
