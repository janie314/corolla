use axum::Json;
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, Row, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug)]
pub enum Error {
    SQL(sqlx::Error),
    QueryDoesNotExist,
    WrongNumberOfArgs,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::SQL(e)
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    sql_template: String,
    args: Vec<String>,
}

#[derive(Clone)]
pub struct DB {
    conn: Arc<RwLock<Pool<Sqlite>>>,
    queries: HashMap<String, Query>,
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
        query("create table if not exists t (c text);")
            .execute(&conn)
            .await?;
        let conn = Arc::new(RwLock::new(conn));
        let mut queries: HashMap<String, Query> = HashMap::new();
        queries.insert(
            "q1".to_string(),
            Query {
                sql_template: "select c from t where c != (?);".to_string(),
                args: Vec::from(["val".to_string()]),
            },
        );
        let db = DB { conn, queries };
        Ok(db)
    }
    pub async fn read_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
    ) -> Result<(), Error> {
        let query = self
            .queries
            .get(query_name)
            .ok_or_else(|| Error::QueryDoesNotExist)?;
        if args.keys().len() == query.args.len() {
            let conn = self.conn.write().await;
            match self.queries.get(query_name) {
                Some(req_query) => {
                    let mut q = sqlx::query(&req_query.sql_template);
                    match args.get("val") {
                        Some(val) => {
                            println!("a");
                            q = q.bind(val);
                        }
                        None => {
                            println!("b");
                            q = q.bind("");
                        }
                    }
                    let res = q.fetch_one(conn.deref()).await?;
                    println!("{:?}", res.try_get::<String, usize>(0).unwrap_or_default());
                    Ok(())
                }
                None => Err(Error::QueryDoesNotExist),
            }
        } else {
            Err(Error::WrongNumberOfArgs)
        }
    }
}
