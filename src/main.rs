#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;
use std::path::PathBuf;

mod parser;
use parser::parse;

mod formatted_metrics;
mod metrics;
mod report;

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

    #[arg(
        short,
        default_value_t = 0,
        help = "Estimated genome size to compute NGX metrics."
    )]
    genome_size: i64,

    #[arg(
        short,
        long,
        default_value_t = 33,
        help = "Phred quality offset (usually 33 or 64)"
    )]
    quality: u8,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Activate parsable mode (csv format with metrics as rows)"
    )]
    csv: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Activate parsable mode (csv format with metrics as columns)"
    )]
    parsable: bool,
}

fn main() {
    let args = Args::parse();
    parse(
        &args.files,
        args.min_size,
        args.genome_size,
        args.quality,
        args.parsable,
        args.csv,
    );
}
