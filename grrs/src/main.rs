use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
  /// The pattern to look for
  pattern: String,
  /// The path to the file to read
  #[clap(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let result = std::fs::read_to_string("test.txt");
  let content = match result {
      Ok(content) => { content },
      Err(error) => { return Err(error.into()); }
  };
  println!("file content: {}", content);
  Ok(())
}