use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {

  let listener = TcpListener::bind("127.0.0.1:8964").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    //println!("Connection establised...");
    handle_connection(stream);
  }
}


fn handle_connection( mut stream: TcpStream){
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();
  // Test1:
  //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
 
  //  Test2:
  // let response = "HTTP/1.1 200 OK\r\n\r\n";
  // stream.write(response.as_bytes()).unwrap();
  // stream.flush().unwrap();

  // 
  let contents = fs::read_to_string("index.html").unwrap();
  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents
  );
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}