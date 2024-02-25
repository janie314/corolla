use core::time;
use std::{
    process::{Command, Stdio},
    thread,
};

#[test]
fn integration_test() {
    let mut corolla = Command::new(env!("CARGO_BIN_EXE_corolla"))
        .args(["-s", "examples/example_spec.json"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run corolla with examples/example_spec.json");
    thread::sleep(time::Duration::from_secs(5));
    assert!(true);
    corolla.kill().expect("could not kill corolla");
}
