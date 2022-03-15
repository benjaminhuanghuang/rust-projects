use assert_cmd::Command;

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("cli-test").unwrap();
  //cmd.assert().success();
  cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
  let mut cmd = Command::cargo_bin("true").unwrap();
  cmd.assert().success();
}

#[test]
fn false_not_ok() {
  let mut cmd = Command::cargo_bin("false").unwrap();
  cmd.assert().failure();
}
