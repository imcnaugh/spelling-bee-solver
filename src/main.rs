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

    let (outside_chars, inside_char) = nyt_spelling_bee_solver::sanitize_and_validate(outside_letters, args.inside_letter);

    let p = nyt_spelling_bee_solver::get_permutations(outside_chars, inside_char);

    println!("{:?}", p);
}