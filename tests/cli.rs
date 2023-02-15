use assert_cmd::Command;

#[test]
fn it_runs() {
    let mut cmd = Command::cargo_bin("rle").unwrap();
    cmd.arg("it_runs");
    cmd.assert().success();
}

#[test]
fn fails_if_no_args() {
    let mut cmd = Command::cargo_bin("rle").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::prelude::predicate::str::contains("Usage"));
}
