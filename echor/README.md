




```
  cargo run -- -n Hello world

  # run echor and redirect channel 1 (STDOUT) to a file called out 
  # and channel 2 (STDERR) to a fil called err
  cargo run 1>out 2>err
```


## Test

```
[dev-dependencies]
assert_cmd = "2"
predicates = "2"
```

run all the tests with names containing the string dies.
```
cargo test dies
````