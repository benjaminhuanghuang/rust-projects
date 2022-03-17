use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

/*
  describes the names and types of the arguments
*/
#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("Ken Youens-Clark <kyclark@gmail.com>")
    .about("Rust cat")
    .arg(
      Arg::with_name("files")
        .value_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("number")
        .short("n")
        .long("number")
        .help("Number lines")
        .takes_value(false)
        .conflicts_with("number_nonblank"),
    )
    .arg(
      Arg::with_name("number_nonblank")
        .short("b")
        .long("number-nonblank")
        .help("Number non-blank lines")
        .takes_value(false),
    )
    .get_matches();

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    number_lines: matches.is_present("number"),
    number_nonblank_lines: matches.is_present("number_nonblank"),
  })
}

// --------------------------------------------------
/*
  Both a filehandle and std::io::stdin implement the BufRead trait
  We can use BufRead::lines function to produce lines of text.
  Note that BufRead::lines will remove any line endings, such as \r\n on Windows and \n on Unix.
*/
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    // filename is a dash (-), read from std::io::stdin
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    // open the given file or propagate an error
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
  for filename in config.files {
    match open(&filename) {
      Err(e) => eprintln!("{}: {}", filename, e),
      Ok(file) => {
        let mut last_num = 0;
        for (line_num, line_result) in file.lines().enumerate() {
          let line = line_result?;
          if config.number_lines {
            println!("{:6}\t{}", line_num + 1, line);
          } else if config.number_nonblank_lines {
            if !line.is_empty() {
              last_num += 1;
              println!("{:6}\t{}", last_num, line);
            } else {
              println!();
            }
          } else {
            println!("{}", line);
          }
        }
      }
    }
  }
  Ok(())
}
