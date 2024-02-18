use super::error::Error;
use log::info;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, fs};

pub type Version = Vec<u64>;

/// Compares two `Version` values.
///
/// Arguments:
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
    Ordering::Equal
}

/// Conversions a `Version` object into a string.
/// E.g. [1,2,3] becomes "1.2.3".
/// TODO: make this a `.to_string()` method?
pub fn version2str(u: &Version) -> String {
    u.into_iter()
        .fold("".to_string(), |a, b| format!("{a}.{b}"))
}

/// Inverse of version2str.
pub fn str2version(u: &str) -> Version {
    u.to_string()
        .split('.')
        .map(|i| i.parse::<u64>().unwrap_or_default())
        .collect()
}

#[derive(Serialize, Deserialize, Clone)]
struct QueryArg {
    pub arg: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
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
    /// The columns a read query returns (should be Some for read queries; None for write queries)
    pub cols: Option<Vec<String>>,
}

/// Represents a DB conversion, which will be executed upon startup if the current DB version is <= Conversion.max.
/// If the conversion is executed, the current DB version will become Conversion.new_version.
#[derive(Serialize, Deserialize)]
pub struct Conversion {
    pub max: Version,
    pub new_version: Version,
    pub queries: Vec<String>,
}

/// The spec.json format, in Rust struct form.
#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub version: Version,
    pub init: Vec<String>,
    pub queries: HashMap<String, Query>,
    pub conversions: Vec<Conversion>,
}

/// Reads a spec.json file into a `Spec` object.
pub fn read_spec(path: &str) -> Result<Spec, Error> {
    info!("reading spec file {path}");
    let file = fs::File::open(path)?;
    let spec: Spec = serde_json::from_reader(file)?;
    Ok(spec)
}
