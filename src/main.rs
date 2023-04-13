use std::io::{stdin, stdout, Write};

/// A verb is either unknown or known.
///
/// Represents the first word in a command.
#[derive(Debug, PartialEq, Eq)]
enum Verb {
    /// The verb is unknown.
    Unknown,

    /// The verb is known and the data is the index of the verb in the
    /// verbs array.
    Known(u32),
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    let verbs = vec!["quit"];

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let words = input
            .split_whitespace()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>();

        // Find the verb.
        let verb = if words.len() == 0 {
            Verb::Unknown
        } else {
            let verb = &words[0];
            if let Some(index) = verbs.iter().position(|v| v == verb) {
                Verb::Known(index as u32)
            } else {
                Verb::Unknown
            }
        };

        println!("{:?}", words);
        println!("{:?}", verb);

        // Quit the game.
        if verb == Verb::Known(0) {
            break;
        }
    }
}
