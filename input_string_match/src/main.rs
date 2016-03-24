use std::io;

fn main() {
    let mut line: String = String::new();
    let stdin: io::Stdin = io::stdin();
    let res: io::Result<usize>;

    res = stdin.read_line(&mut line);

    match res {
        Ok(nbytes) => {
            println!("{}({} bytes read)", line, nbytes);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

