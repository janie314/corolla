use crate::error::Error;
use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, Row, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Query {
    sql_template: String,
    args: Vec<String>,
}

#[derive(Clone)]
pub struct DBQuery {
    conn: Arc<RwLock<Pool<Sqlite>>>,
    queries: HashMap<String, Query>,
}

impl DBQuery {
    pub async fn new(
        filepath: &str,
        init_statements: &[&str],
        queries: &[(&str, &str, Vec<String>)],
    ) -> Result<Self, Error> {
        let conn = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(filepath)
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await?;
        for s in init_statements {
            query(s).execute(&conn).await?;
        }
        let conn = Arc::new(RwLock::new(conn));
        let mut queries_aux: HashMap<String, Query> = HashMap::new();
        for (name, statement, args) in queries {
            queries_aux.insert(
                name.to_string(),
                Query {
                    sql_template: statement.to_string(),
                    args: args.to_vec(),
                },
            );
        }
        let db = DBQuery {
            conn,
            queries: queries_aux,
        };
        Ok(db)
    }
    pub async fn read_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
    ) -> Result<Json<Vec<String>>, Error> {
        let query = self
            .queries
            .get(query_name)
            .ok_or_else(|| Error::QueryDoesNotExist)?;
        if args.keys().len() == query.args.len() {
            let conn = self.conn.read().await;
            let mut statement = sqlx::query(&query.sql_template);
            for arg in query.args.iter() {
                match args.get(arg) {
                    Some(val) => {
                        statement = statement.bind(val);
                    }
                    None => {
                        statement = statement.bind("");
                    }
                }
            }
            let sql_res = statement.fetch_one(conn.deref()).await?;
            let mut res = Vec::<String>::new();
            for c in 0..(sql_res.columns().len()) {
                res.push(sql_res.try_get::<String, usize>(c).unwrap_or_default());
            }
            Ok(Json(res))
        } else {
            Err(Error::WrongNumberOfArgs)
        }
    }
}
