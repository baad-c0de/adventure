use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        println!("{}", input);
    }
}
