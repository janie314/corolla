use core::time;
use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use std::{
    collections::HashMap,
    fmt::format,
    process::{Command, Stdio},
    thread,
};

#[test]
fn integration_test() {
    let mut corolla = Command::new(env!("CARGO_BIN_EXE_corolla"))
        .args(["-s", "examples/example_spec.json"])
        //.stderr(Stdio::null())
        //.stdout(Stdio::null())
        .spawn()
        .expect("failed to run corolla with examples/example_spec.json");
    thread::sleep(time::Duration::from_secs(5));
    let mut body = HashMap::new();
    body.insert("a", "cargo test 90210");
    let client = reqwest::blocking::Client::new();
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
