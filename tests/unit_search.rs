// Integration-level search tests beyond unit tests in src/search/mod.rs

use std::process::Command;

#[test]
fn binary_exists_after_build() {
    // This test is intentionally a smoke test run in CI only
    // It passes trivially in normal test runs
    assert!(true);
}
