#![allow(unused)]

use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use knuffel::{parse, Decode};

/// A verb is either unknown or known.
///
/// Represents the first word in a command.
#[derive(Debug, PartialEq, Eq)]
enum VerbReference {
    /// No verbs were given.
    None,

    /// The verb is unknown.
    Unknown,

    /// The verb is known and the data is the index of the verb in the
    /// verbs array.
    Known(u32),
}

#[derive(Decode, Debug)]
enum Database {
    Verbs(Verbs),
}

#[derive(Decode, Debug)]
struct Verbs {
    #[knuffel(children(name = "verb"))]
    pub verbs: Vec<Verb>,
}

#[derive(Decode, Debug)]
struct Verb {
    #[knuffel(argument)]
    pub index: u32,
    #[knuffel(argument)]
    pub text: String,
}

fn main() {
    // Read the KDL file
    let kdl = read_to_string("data.kdl").unwrap();
    let data = parse::<Vec<Database>>("data.kdl", &kdl).unwrap();

    let mut verb_map = HashMap::new();
    for data in data {
        match data {
            Database::Verbs(verbs) => {
                for verb in verbs.verbs {
                    verb_map.insert(verb.text, verb.index);
                }
            }
        }
    }

    let stdin = stdin();
    let mut stdout = stdout();

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
            VerbReference::None
        } else {
            let verb = &words[0];

            verb_map
                .get(verb)
                .map(|i| VerbReference::Known(*i))
                .unwrap_or(VerbReference::Unknown)
        };

        match verb {
            VerbReference::None => println!("Pardon?"),
            VerbReference::Unknown => println!("I don't understand that."),
            VerbReference::Known(0) => {
                println!("Goodbye!");
                return;
            }
            VerbReference::Known(index) => println!("Verb index: {}", index),
        }
    }
}
