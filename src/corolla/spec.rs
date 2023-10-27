use super::error::Error;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, fs};

type Version = Vec<u64>;

/// Compares two `Version` values.
///
/// # Arguments
/// * `u`, `v` - References to `Version` types.
pub fn version_cmp(u: &Version, v: &Version) -> Ordering {
    for (a, b) in u.iter().zip(v) {
        if a < b {
            return Ordering::Less;
        }
        if a > b {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

#[derive(Serialize, Deserialize, Clone)]
struct QueryArg {
    pub arg: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum QueryPart {
    SQL(String),
    Arg(QueryArg),
}

/// Represents a database query.
#[derive(Serialize, Deserialize, Clone)]
pub struct Query {
    /// [A SQLite statement with parameters.](https://www.sqlite.org/c3ref/bind_blob.html) Only `?` parameters are tested.
    pub sql_template: String,
    /// The query's list of parameter names, in order.
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Conversion {
    pub min: Version,
    pub new_version: Version,
    pub queries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub version: Version,
    pub init: Vec<String>,
    pub queries: HashMap<String, Query>,
    pub conversions: Vec<Conversion>,
}

pub fn read_spec(path: &str) -> Result<Spec, Error> {
    let file = fs::File::open(path)?;
    let spec: Spec = serde_json::from_reader(file)?;
    Ok(spec)
}
