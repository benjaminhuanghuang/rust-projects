# Rust head
From  Command-Line Rust: A Project-Based Primer for Writing Rust CLIs, Ch 4
 https://github.com/kyclark/command-line-rust


implement the head program, which will print the first few lines or bytes of one
or more files

-n option allows you to control the number of lines that are shown

-c option shows only the given number of bytes

With no file arguments, head will read from STDIN
```
  $ cat tests/inputs/ten.txt | head -n 2
```


