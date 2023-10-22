use crate::error::Error;
use axum::Json;
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, Row, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct Query {
    sql_template: String,
    args: Vec<String>,
}

#[derive(Clone)]
pub struct DB {
    conn: Arc<RwLock<Pool<Sqlite>>>,
    queries: HashMap<String, Query>,
}

impl DB {
    /// Construct a new DB object, which consists of a pooled SQLite connection wrapped by a read/write lock and a query lookup.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Filepath to the SQLite database.
    /// * `init_statements` - A list of SQL statements that will be executed to initialize the datbase, in order.
    /// * `queries` - A lookup table of SQL queries.
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
        let db = DB {
            conn,
            queries: queries_aux,
        };
        Ok(db)
    }
    /// Executes a read-only query on the SQLite database and returns the result.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The code name of the query in the query lookup table.
    /// * `args` - Arguments to be bound to the query.
    pub async fn read_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
    ) -> Result<Json<Vec<Vec<String>>>, Error> {
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
            let sql_res = statement.fetch_all(conn.deref()).await?;
            let mut res = Vec::<Vec<String>>::new();
            for row in sql_res {
                let mut v = Vec::<String>::new();
                for c in 0..(row.len()) {
                    v.push(row.try_get::<String, usize>(c).unwrap_or_default());
                }
                res.push(v);
            }
            Ok(Json(res))
        } else {
            Err(Error::WrongNumberOfArgs)
        }
    }
    /// Executes a write-only query on the SQLite database and returns the result.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The code name of the query in the query lookup table.
    /// * `args` - Arguments to be bound to the query.
    pub async fn write_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
    ) -> Result<(), Error> {
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
            statement.execute(conn.deref()).await?;
            Ok(())
        } else {
            Err(Error::WrongNumberOfArgs)
        }
    }
}
