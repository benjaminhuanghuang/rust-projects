fn main() {
  // If either get_args or run returns an Err, print it to STDERR.
  if let Err(e) = catr::get_args().and_then(catr::run) {
    eprintln!("{}", e);
    std::process::exit(1);
  }
}
