#  Rust cat

From  Command-Line Rust: A Project-Based Primer for Writing Rust CLIs, Ch 3
 https://github.com/kyclark/command-line-rust




 ## Run 
 ```
 cargo run --quiet -- --help

 # read text from STDIN
 cat tests/inputs/fox.txt | cargo run
 cargo run -- - < tests/inputs/fox.txt
 
 ```