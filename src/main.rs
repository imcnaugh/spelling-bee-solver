use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Letters in white on the outside of the puzzle.
    #[arg(value_parser = validate_outside_letters)]
    outside_letters: Vec<char>,

    // /// The letter in yellow in the center of the puzzle.
    // #[arg(short, long)]
    // inside_letter: char,
}

fn main() {
    let _args = Args::parse();
}


fn validate_outside_letters(s: &str) -> Result<Vec<char>, String> {
    let s = s.to_lowercase();
    let chars: Vec<char> = s.chars().collect();

    let unique_chars: HashSet<_> = chars.iter().collect();
    if unique_chars.len() != 6 {
        return Err(format!(
            "I should have 6 unique characters on the outside of the puzzle, I received: {}",
            unique_chars.len()
        ));
    }

    for char in &chars {
        if !char.is_alphabetic() {
            return Err(format!(
                "{} does not appear to be a letter between a and z",
                char
            ));
        }
    }

    Ok(chars)
}