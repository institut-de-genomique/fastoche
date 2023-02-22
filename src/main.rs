use clap::Parser;

use fastoche;


/// fastoche is a rdbioseq program
#[derive(Parser)]
struct Args {
    /// first argument, positional
    first: String,
    /// second argument
    #[clap(short, long)]
    second: String,
}


fn main() {
    // parse cli
    let args = Args::parse();

    // ...
}
