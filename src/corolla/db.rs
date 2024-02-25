use super::{
    error::Error,
    spec::{Query, Spec},
    version::{InstanceVersion, Version},
};
use log::{debug, info};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Pool, Row, Sqlite, SqlitePool,
};
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

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
    /// Arguments:
    ///
    /// * `db` - Filepath to the SQLite database.
    /// * `spec` - Filepath to the spec.json
    pub async fn from_spec(db: &str, spec: &Spec) -> Result<Self, Error> {
        info!("opening sqlite db {db}");
        let conn = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .create_if_missing(true)
                .filename(db)
                .journal_mode(SqliteJournalMode::Wal),
        )
        .await?;
        debug!("init DB object");
        let conn = Arc::new(RwLock::new(conn));
        let queries = spec.queries.clone();
        let cols = HashMap::<String, Vec<String>>::new();
        let db = DB {
            conn,
            queries,
            cols,
        };
        info!("checking if corolla DB has been initialized");
        // if DB is initialized (can find a corolla instance version inside it), then run the conversions
        match db._instance_version().await? {
            Some(v) => db._run_conversions(spec, v).await?,
            _ => (),
        };
        info!("running init statements from spec");
        for s in &spec.init {
            db.write_raw_query(&s, None).await?;
        }
        info!("initializing corolla db tables");
        let _ = db._init_corolla_tables(&spec).await?;
        Ok(db)
    }
    /// Executes a read-only query on the SQLite database and returns the result.
    ///
    /// Arguments:
    ///
    /// * `query_name` - The code name of the query in the query lookup table.
    /// * `args` - Arguments to be bound to the query.
    /// * `conn` - Can pass a `conn.read()` here to execute this method with a shared lock.
    pub async fn read_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
        conn: Option<RwLockReadGuard<'_, Pool<Sqlite>>>,
    ) -> Result<Vec<Vec<String>>, Error> {
        let query = self
            .queries
            .get(query_name)
            .ok_or_else(|| Error::QueryDoesNotExist)?;
        if args.keys().len() == query.args.len() {
            let conn = match conn {
                Some(c) => {
                    debug!("using shared read lock");
                    c
                }
                None => {
                    debug!("waiting for read lock");
                    self.conn.read().await
                }
            };
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
    /// Arguments:
    ///
    /// * `sql` - SQL statement to execute
    /// * `args` - Arguments to be bound to the query.
    /// * `conn` - Can pass a `conn.read()` here to execute this method with a shared lock.
    async fn _read_raw_query(
        &self,
        sql: &str,
        conn: Option<RwLockReadGuard<'_, Pool<Sqlite>>>,
    ) -> Result<Vec<Vec<String>>, Error> {
        let conn = match conn {
            Some(c) => {
                debug!("using shared read lock");
                c
            }
            None => {
                debug!("waiting for read lock");
                self.conn.read().await
            }
        };
        debug!("executing sql statement {sql}");
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
    /// Executes a read-only query on the SQLite database and returns a single row.
    ///
    /// Arguments:
    ///
    /// * `sql` - SQL statement to execute
    /// * `args` - Arguments to be bound to the query.
    /// * `conn` - Can pass a `conn.read()` here to execute this method with a shared lock.
    pub async fn _read_one_raw_query(
        &self,
        sql: &str,
        conn: Option<RwLockReadGuard<'_, Pool<Sqlite>>>,
    ) -> Result<Vec<String>, Error> {
        let conn = match conn {
            Some(c) => {
                debug!("using shared read lock");
                c
            }
            None => {
                debug!("waiting for read lock");
                self.conn.read().await
            }
        };
        debug!("executing sql statement {sql}");
        let statement = sqlx::query(sql);
        let row = statement.fetch_one(conn.deref()).await?;
        let mut res: Vec<String> = vec![];
        for c in 0..(row.len()) {
            res.push(row.try_get::<String, usize>(c).unwrap_or_default());
        }
        Ok(res)
    }
    /// Executes a write-only query on the SQLite database and returns the result.
    ///
    /// Arguments:
    ///
    /// * `query_name` - The code name of the query in the query lookup table.
    /// * `args` - Arguments to be bound to the query.
    /// * `conn` - Can pass a `conn.write()` here to execute this method with a shared lock.
    pub async fn write_query(
        &self,
        query_name: &str,
        args: &HashMap<String, String>,
        conn: Option<RwLockWriteGuard<'_, Pool<Sqlite>>>,
    ) -> Result<(), Error> {
        let query = self
            .queries
            .get(query_name)
            .ok_or_else(|| Error::QueryDoesNotExist)?;
        if args.keys().len() == query.args.len() {
            let conn = match conn {
                Some(c) => {
                    debug!("using shared write lock");
                    c
                }
                None => {
                    debug!("waiting for write lock");
                    self.conn.write().await
                }
            };
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
    /// Arguments
    ///
    /// * `sql` - SQL statement to execute
    /// * `conn` - Can pass a `conn.write()` here to execute this method with a shared lock.
    /// TODO: This needs to take a vector of SQL statements
    async fn write_raw_query(
        &self,
        sql: &str,
        conn: Option<RwLockWriteGuard<'_, Pool<Sqlite>>>,
    ) -> Result<(), Error> {
        let conn = match conn {
            Some(c) => {
                debug!("using shared write lock");
                c
            }
            None => {
                debug!("waiting for write lock");
                self.conn.write().await
            }
        };
        debug!("executing sql statement {sql}");
        sqlx::query(&sql).execute(conn.deref()).await?;
        Ok(())
    }
    /// Initialize core Corolla sqlite tables
    async fn _init_corolla_tables(&self, spec: &Spec) -> Result<(), Error> {
        self.write_raw_query(
            "create table if not exists corolla_db_info (key text, value text);",
            None,
        )
        .await?;
        let version_str: String = spec.version.clone().into();
        self.write_raw_query(
            &format!(
                "insert into corolla_db_info values ('version', '{}');",
                version_str
            ),
            None,
        )
        .await?;
        Ok(())
    }
    /// Get current DB instance version
    async fn _instance_version(&self) -> Result<Option<InstanceVersion>, Error> {
        let res = self
            ._read_one_raw_query(
                "select value from corolla_db_info where key = 'version';",
                None,
            )
            .await;
        match res {
            Ok(res) => match res.get(0) {
                Some(val) => Ok(Some(InstanceVersion::from(val))),
                None => Ok(None),
            },
            Err(err) => match err {
                Error::SQL(err) => {
                    if err
                        .as_database_error()
                        .is_some_and(|dberr| dberr.message().contains("no such table"))
                    {
                        Ok(None)
                    } else {
                        Err(Error::SQL(err))
                    }
                }
                _ => Err(err),
            },
        }
    }

    /// Run conversions specified by a spec.json file
    async fn _run_conversions(&self, spec: &Spec, v: Version) -> Result<(), Error> {
        info!("running DB conversions");
        for (i, conversion) in spec.conversions.iter().enumerate() {
            if v <= conversion.max_version {
                info!("running conversion {i}");
                for query in &conversion.queries {
                    self.write_raw_query(&query, None).await?;
                }
            } else {
                info!("skipping conversion {i}");
            }
        }
        todo!()
    }
}
