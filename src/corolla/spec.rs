use super::{error::Error, version::InstanceVersion};
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

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
    pub max: InstanceVersion,
    pub new_version: InstanceVersion,
    pub queries: Vec<String>,
}

/// The spec.json format, in Rust struct form.
#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub version: InstanceVersion,
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
