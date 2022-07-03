#[test]
fn will_success() {
    assert!(true);
}


#[test]
fn will_fail() {
    assert!(false);
}

#[test]
fn runs() {
    use std::process::Command;
    let mut cmd = Command::new("hello_world");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_assert_cmd() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn check_tomorrow() {
    use assert_cmd::Command;
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("tomorrow is well\n");
}