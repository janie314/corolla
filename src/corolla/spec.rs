use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use super::error::Error;

type Version = Vec<u64>;

#[derive(Serialize, Deserialize)]
struct QueryArg {
    arg: String,
}

#[derive(Serialize, Deserialize)]
enum QueryPart {
    SQL(String),
    Arg(QueryArg),
}

#[derive(Serialize, Deserialize)]
struct Query {
    friendly_name: String,
    query: Vec<QueryPart>,
}

#[derive(Serialize, Deserialize)]
struct Conversion {
    // TODO: This may be stupid. use a string
    min: Version,
    new_version: Version,
    queries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    version: Version,
    init: Vec<String>,
    queries: HashMap<String, Query>,
    conversions: Vec<Conversion>,
}

pub fn read_spec(path: &str) -> Result<Spec, Error> {
    let file = fs::File::open(path)?;
    let spec: Spec = serde_json::from_reader(file)?;
    Ok(spec)
}
