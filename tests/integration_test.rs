use core::time;
use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

fn cleanup(path: &str, pid: Option<u32>) {
    for file in [
        "corolla-integration-test.sqlite3",
        "corolla-integration-test.sqlite3-shm",
        "corolla-integration-test.sqlite3-wal",
    ]
    .iter()
    {
        Command::new("rm")
            .arg(Path::new(path).join(file).to_str().unwrap())
            .output()
            .expect("could not execute cleanup step");
    }
    match pid {
        Some(pid) => {
            Command::new("kill")
                .arg(pid.to_string())
                .output()
                .expect("could not kill corolla; this will require manual cleanup");
        }
        None => (),
    }
}

fn get_root_dir() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_corolla"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn start_server(spec_path: &Path) {
    let root_dir = get_root_dir();
    let corolla = Command::new(env!("CARGO_BIN_EXE_corolla"))
        .args([
            "-s",
            spec_path.to_str().unwrap(),
            "-d",
            Path::new(&root_dir)
                .join("tmp")
                .join("corolla-integration-test.sqlite3")
                .to_str()
                .unwrap(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run corolla with examples/example_spec.json");
}

#[test]
fn integration_test() {
    cleanup(path, None);
    let corolla = Command::new(env!("CARGO_BIN_EXE_corolla"))
        .args([
            "-s",
            "examples/example_spec.json",
            "-d",
            Path::new(path)
                .join("tmp")
                .join("corolla-integration-test.sqlite3")
                .to_str()
                .unwrap(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run corolla with examples/example_spec.json");
    thread::sleep(time::Duration::from_secs(10));
    let inputs = ["1-2-3", "do-re-mi", "baby you and me"];
    let client = reqwest::blocking::Client::new();
    for x in inputs.iter() {
        let mut body = HashMap::new();
        body.insert("a", x);
        let res = client
            .post("http://localhost:50000/write/write01")
            .json(&body)
            .send()
            .expect("could not make HTTP request");
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "HTTP request failed with message {:?}",
            res.text()
        );
    }
    let res: Vec<Vec<String>> = reqwest::blocking::get("http://localhost:50000/read/read01")
        .expect("could not perform GET curl")
        .json()
        .expect("could not parse JSON into expected structure");
    for (i, row) in res.iter().enumerate() {
        assert_eq!(row.len(), 1);
        if i == 0 {
            assert_eq!(row.get(0).unwrap(), "c");
        } else {
            println!("{i} {}", row.get(0).unwrap_or(&String::from("")));
            let x = inputs.get(i - 1).unwrap();
            assert_eq!(row.get(0).unwrap(), x);
        }
    }
    Command::new("kill")
        .arg(corolla.id().to_string())
        .output()
        .expect("could not kill corolla; this will require manual cleanup");
    let corolla = Command::new(env!("CARGO_BIN_EXE_corolla"))
        .args(["-s", "examples/example_spec_with_conversions.json"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run corolla with examples/example_spec.json");
    thread::sleep(time::Duration::from_secs(2));
    let inputs = [
        ("cargo test 90210", "what we have here"),
        ("cargo test 90211", "is failure to communicate"),
        ("cargo test 90212", "some men you just can't reach"),
    ];
    for (x, y) in inputs.iter() {
        let mut body = HashMap::new();
        body.insert("a", x);
        body.insert("b", y);
        let res = client
            .post("http://localhost:50000/write/write01")
            .json(&body)
            .send()
            .expect("could not make HTTP request");
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "HTTP request failed with message {:?}",
            res.text()
        );
    }
    let res: Vec<Vec<String>> = reqwest::blocking::get("http://localhost:50000/read/read01")
        .expect("could not perform GET curl")
        .json()
        .expect("could not parse JSON into expected structure");
    let mut iter = res.iter();
    assert_eq!(
        iter.next().unwrap(),
        &vec!["c".to_string(), "newcol".to_string()]
    );
    iter.next();
    iter.next();
    iter.next();
    for (i, row) in iter.enumerate() {
        assert_eq!(row.len(), 2);
        let (x, y) = inputs.get(i).unwrap();
        assert_eq!(row.get(0).unwrap(), x);
        assert_eq!(row.get(1).unwrap(), y);
    }
    cleanup(path, Some(corolla.id()));
}
