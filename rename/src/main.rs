use std::fs;
use std::path::Path;
use std::error::Error;

fn main() {
    let path = Path::new("./");
    //fs::read_dir() read entities
    for entry in fs::read_dir(path).expect("Path doest not exist.") {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            let new_filename = format!("{}.jpg", filename);
            // rename file
            match fs::rename(filename, &new_filename) {
                Err(why) => panic!("{} => {}: {}", filename, new_filename, why.description()),
                Ok(_) => println!("{} => {}", filename, new_filename),
            }
        }
    }
}