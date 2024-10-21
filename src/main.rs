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

    println!("{}", args.outside_letters);
}