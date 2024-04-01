use std::process::{Command, Stdio};

#[test]
fn api_test() {
    let mut bun_test = Command::new("bun")
        .args(["test"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to run `bun test`");
    assert!(bun_test
        .wait()
        .expect("failed to wait for `bun test`")
        .success())
}
