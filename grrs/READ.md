# 通过写一个十五分钟的命令行应用学习 Rust
- https://rust-cli.github.io/book/tutorial/cli-args.html
- https://learnku.com/docs/rust-cla-2020/learn-rust-by-writing-a-15-minute-command-line-application/10260


编写 grep clone app , 称为 grrs （发音“grass” ）。
```
$ grrs foobar test.txt
```
grrs查看 test.txt 并且打印出含有 foobar 的行



## 获取参数
```
std::env::args() 
```

解析命令行参数最常用的库叫做 clap
```
clap = { version = "3.0", features = ["derive"] }


use clap::Parser;

// parse the arguments into our struct
let args = Cli::parse();
```