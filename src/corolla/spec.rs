use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    min: f64,
    new_version: f64,
    queries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Spec {
    version: u8,
    init: Vec<String>,
    queries: HashMap<String, Query>,
    conversions: Vec<Conversion>,
}
