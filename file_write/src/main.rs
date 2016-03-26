// Write some data into a file
// Usage: cargo run <file path> string

use std::io::Write;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: cargo run <file path> string");
    }
    let file_name = args[1].clone();
    let contents = args[2].clone();

    match File::create(file_name) {
        Ok(mut f) => {
            match f.write(contents.as_bytes()) {
                Ok(_) => {}
                Err(error) => {
                    println!("Error writing to file: {}", error);
                }
            }
        }
        Err(error) => {
            println!("Error when creating file for writing: {}", error);
        }
    }
}
