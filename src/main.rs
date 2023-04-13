use std::io::{stdin, stdout, Write};

/// A verb is either unknown or known.
///
/// Represents the first word in a command.
#[derive(Debug, PartialEq, Eq)]
enum Verb {
    /// No verbs were given.
    None,

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
            Verb::None
        } else {
            let verb = &words[0];
            if let Some(index) = verbs.iter().position(|v| v == verb) {
                Verb::Known(index as u32)
            } else {
                Verb::Unknown
            }
        };

        match verb {
            Verb::None => println!("Pardon?"),
            Verb::Unknown => println!("I don't understand that."),
            Verb::Known(0) => {
                println!("Goodbye!");
                return;
            }
            _ => unreachable!(),
        }
    }
}
