use super::{
    error::Error,
    version::{InstanceVersion, SpecVersion},
};
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

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

/// Represents a read-only database query (returns rows, does not change DB).
#[derive(Serialize, Deserialize, Clone)]
pub struct ReadQuery {
    /// [A SQLite statement with parameters.](https://www.sqlite.org/c3ref/bind_blob.html) Only `?` parameters are tested.
    pub sql_template: String,
    /// The query's list of parameter names, in order.
    pub args: Vec<String>,
    /// The columns the query results will use
    pub cols: Vec<String>,
}

/// Represents a write-only database query (can return rows, changes DB).
#[derive(Serialize, Deserialize, Clone)]
pub struct WriteQuery {
    /// [A SQLite statement with parameters.](https://www.sqlite.org/c3ref/bind_blob.html) Only `?` parameters are tested.
    pub sql_template: String,
    /// The query's list of parameter names, in order.
    pub args: Vec<String>,
}

/// Represents a DB conversion, which will be executed upon startup if the current DB version is <= Conversion.max.
/// If the conversion is executed, the current DB version will become Conversion.new_version.
#[derive(Serialize, Deserialize)]
pub struct Conversion {
    pub max_version: InstanceVersion,
    pub new_version: InstanceVersion,
    pub queries: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Queries {
    pub read: HashMap<String, ReadQuery>,
    pub write: HashMap<String, WriteQuery>,
}

/// The spec.json format, in Rust struct form.
#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub spec_version: SpecVersion,
    pub version: InstanceVersion,
    pub init: Vec<String>,
    pub queries: Queries,
    pub conversions: Vec<Conversion>,
}

/// Reads a spec.json file into a `Spec` object.
pub fn read_spec<P>(path: P) -> Result<Spec, Error>
where
    P: AsRef<Path>,
{
    info!("reading spec file");
    let file = fs::File::open(path)?;
    let spec: Spec = serde_json::from_reader(file)?;
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corolla::version::Version;
    use pretty_assertions::assert_eq;
    use std::{env, path::Path};

    #[test]
    /// versions convert to strings appropriately
    fn read_and_parse_spec() {
        let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let spec_path = Path::new(&proj_dir).join("examples/example_spec.json");
        let spec = read_spec(spec_path).unwrap();
        assert_eq!(spec.version, Version::from([1, 0, 1]));
        assert_eq!(
            spec.init.get(0).unwrap(),
            "create table if not exists t (c text);"
        );
        let read_query = spec.queries.read.get("read01").unwrap();
        assert_eq!(read_query.sql_template, "select c from t;");
        assert_eq!(read_query.args.len(), 0);
        assert_eq!(read_query.cols.get(0).unwrap(), "c");
        let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let spec_path = Path::new(&proj_dir).join("examples/example_spec_with_conversions.json");
        let spec = read_spec(spec_path).unwrap();
        assert_eq!(spec.version, Version::from([1, 0, 2]));
        assert_eq!(
            spec.init.get(0).unwrap(),
            "create table if not exists t (c text, newcol text);"
        );
        let write_query = spec.queries.write.get("write01").unwrap();
        assert_eq!(write_query.sql_template, "insert into t values (?,?);");
        assert_eq!(write_query.args.len(), 2);
        let conversion = spec.conversions.get(0).unwrap();
        assert_eq!(conversion.max_version, Version::from("1.0.1"));
        assert_eq!(conversion.new_version, Version::from("1.0.2"));
    }
}
