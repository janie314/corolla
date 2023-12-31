use super::{
    consts::SPEC_VERSION,
    error::Error,
    spec::{version2str, Query, Spec},
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, Row, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

/// Represents a connection to a SQLite database.
#[derive(Clone)]
pub struct DB {
    /// A read/write lock on top of a SQLite connection pool.
    conn: Arc<RwLock<Pool<Sqlite>>>,
    /// A lookup table of DB queries.
    queries: HashMap<String, Query>,
    /// A lookup table of read queries' columns.
    cols: HashMap<String, Vec<String>>,
}

impl DB {
    /// Construct a new DB object, which consists of a pooled SQLite connection wrapped by a read/write lock and a query lookup.
    ///
    /// # Arguments
    ///
    /// * `db` - Filepath to the SQLite database.
    /// * `spec` - Filepath to the spec.json
    pub async fn from_spec(db: &str, spec: &Spec) -> Result<Self, Error> {
        let conn = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(db)
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await?;
        for s in &spec.init {
            sqlx::query(&s).execute(&conn).await?;
        }
        let conn = Arc::new(RwLock::new(conn));
        let queries = spec.queries.clone();
        /*
         * run conversions
         */
        // TODO uncomment
        // for conversion in &spec.conversions {}
        let cols = HashMap::<String, Vec<String>>::new();
        let db = DB {
            conn,
            queries,
            cols,
        };
        let _ = db._init_db_info().await?;
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
    ) -> Result<Vec<Vec<String>>, Error> {
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
            if let Some(cols) = &query.cols {
                res.push(cols.clone());
            }
            for row in sql_res {
                let mut v = Vec::<String>::new();
                for c in 0..(row.len()) {
                    v.push(row.try_get::<String, usize>(c).unwrap_or_default());
                }
                res.push(v);
            }
            Ok(res)
        } else {
            Err(Error::WrongNumberOfArgs)
        }
    }
    /// Executes a read-only query on the SQLite database and returns the result.
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL statement to execute
    /// * `args` - Arguments to be bound to the query.
    pub async fn read_raw_query(&self, sql: &str) -> Result<Vec<Vec<String>>, Error> {
        let conn = self.conn.read().await;
        let statement = sqlx::query(sql);
        let sql_res = statement.fetch_all(conn.deref()).await?;
        let mut res = Vec::<Vec<String>>::new();
        for row in sql_res {
            let mut v = Vec::<String>::new();
            for c in 0..(row.len()) {
                v.push(row.try_get::<String, usize>(c).unwrap_or_default());
            }
            res.push(v);
        }
        Ok(res)
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
            let conn = self.conn.write().await;
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
    /// Execute a SQL statement that modifies the database
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL statement to execute
    /// TODO: This needs to take a vector of SQL statements
    pub async fn write_raw_query(&self, sql: &str) -> Result<(), Error> {
        let conn = self.conn.write().await;
        sqlx::query(&sql).execute(conn.deref()).await?;
        Ok(())
    }
    /// Initialize core Corolla sqlite tables
    async fn _init_db_info(&self) -> Result<(), Error> {
        self.write_raw_query("create table if not exists corolla_db_info (key text, value text);")
            .await?;
        self.write_raw_query(&format!(
            "insert into corolla_db_info values ('version', '{}');",
            version2str(&Vec::from(SPEC_VERSION))
        ))
        .await?;
        Ok(())
    }
}
