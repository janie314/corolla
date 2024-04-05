use common::server;
use std::process::Command;

use crate::common::cleanup;

mod common;

#[test]
fn api_test() {
    let mut corolla = server("examples/example_spec.json");
    let mut bun_test = Command::new("bun")
        .args(["test"])
        .spawn()
        .expect("failed to run `bun test`");
    assert!(bun_test
        .wait()
        .expect("failed to wait for `bun test`")
        .success());
    cleanup(true, Some(&mut corolla));
}
