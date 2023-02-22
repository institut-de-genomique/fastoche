use clap::Parser;
use std::path::PathBuf;

mod parser;
use parser::parse;

#[derive(Parser)]
#[command(author="Benjamin Istace",
    about="Computes statistics about Fastx files that are gzipped or not",
    long_about=None
)]
struct Args {
    #[arg(
        short,
        required = true,
        help = "Fastx files to process. Can be gzipped."
    )]
    files: Vec<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Sequences shorter than this number will not be processed."
    )]
    min_size: usize,
}

fn main() {
    let args = Args::parse();
    parse(&args.files);
}
