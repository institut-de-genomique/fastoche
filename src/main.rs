use clap::Parser;
use report::parse_output_format;
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
        help = "Fastx files to process. Can be gzipped. Can be specified multiple times if you need to compute metrics on several files."
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
        help = "Estimated genome size to compute NGX metrics (in bases)."
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
        conflicts_with = "csv",
        help = "Activate parsable mode (csv format with metrics as columns)"
    )]
    parsable: bool,

    #[arg(
        long,
        requires = "parsable",
        help = "(--parsable only) Comma-separated list of metrics to output"
    )]
    output_format: Option<String>,

    #[arg(
        long,
        requires = "parsable",
        requires = "output_format",
        default_value_t = false,
        help = "(--parsable only) Do not print a header"
    )]
    no_header: bool,

    #[arg(
        long,
        help = "Activate per sequence metrics mode. Provide a path to a file to store the metrics. WARNING: does not work for multiple input files."
    )]
    per_seq: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Use these names instead of inferring them. Format name_1,name_2,name_n"
    )]
    rename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let output_fields = parse_output_format(&args.output_format);
    parse(
        &args.files,
        args.min_size,
        args.genome_size,
        args.quality,
        args.parsable,
        args.csv,
        args.per_seq,
        args.rename,
        output_fields,
        args.no_header,
    );
}
