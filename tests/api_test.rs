use crate::common::cleanup;
use common::server;
use std::{env::var, process::Command};

mod common;

#[tokio::test]
async fn api_test() {
    let mut corolla = server("examples/example_spec.json").await;
    let bun = var("BUN_RUNTIME").expect("invalid environment variable BUN_RUNTIME");
    assert_ne!(bun.len(), 0);
    let mut bun_test = Command::new(bun)
        .args(["test"])
        .spawn()
        .expect("failed to run `bun test`");
    assert!(bun_test
        .wait()
        .expect("failed to wait for `bun test`")
        .success());
    cleanup(true, Some(&mut corolla)).await;
}
