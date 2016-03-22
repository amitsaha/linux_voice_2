use std::io;

fn main() {
    let mut line: String = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(nbytes) => {
            println!("{}({} bytes read)", line, nbytes);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

