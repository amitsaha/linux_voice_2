use std::io;

fn main() {
    let mut line: String = String::new();
    let stdin: io::Stdin = io::stdin();
    let res: io::Result<usize>;

    res = stdin.read_line(&mut line);
    if res.is_ok() {
        // Unwrap the result to extract the value
        let nbytes: usize = res.unwrap();
        println!("{}({} bytes read)", line, nbytes);
    } else {
        res.unwrap();
    }
}
