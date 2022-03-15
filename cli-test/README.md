

## Test system command
```
use std::process::Command;
#[test]
fn runs() {
  let mut cmd = Command::new("ls");
  let res = cmd.output();
  assert!(res.is_ok());
}
```

## Test exe in the current crate.
use the crate assert_cmd to find the program in my crate directory.
```

# need crates only for testing and benchmarking:
[dev-dependencies]
assert_cmd = "1"

```

```
use assert_cmd::Command;

#[test]
fn runs() {
  // Create a Command to run hello in the current crate.
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success();
}
```


