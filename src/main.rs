use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Letters in white on the outside of the puzzle.
    #[arg(short, long)]
    pub outside_letters: String,

    /// The letter in yellow in the center of the puzzle.
    #[arg(short, long)]
    inside_letter: char,
}

fn main() {
    let args = Args::parse();

    let outside_letters = args.outside_letters.chars().collect();

    let mut result = nyt_spelling_bee_solver::get_solution(outside_letters, args.inside_letter);

    for word in &mut result {
        if word.len() < 7 {
            continue;
        }

        // check if the word has 7 unique chars
        if word.chars().collect::<HashSet<_>>().len() == 7 {
            word.push_str(" *");
        };
    }

    //sort result by length then alpha
    result.sort_by(|a, b| {
        if a.len() == b.len() {
            b.cmp(a)
        } else {
            a.len().cmp(&b.len())
        }
    });

    result.reverse();

    for w in result {
        println!("{w}");
    }
}