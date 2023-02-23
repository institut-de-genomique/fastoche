use flate2::read::GzDecoder;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

trait FastxParser {}
impl<R> FastxParser for bio::io::fasta::Reader<R> where R: Read {}
impl<R> FastxParser for bio::io::fastq::Reader<R> where R: Read {}

pub fn parse(files: &[PathBuf]) {
    for f in files.iter() {
        compute_stats(f);
    }
}

fn compute_stats(file: &Path) {
    assert!(file.exists(), "File not found {file:?}");

    let mut reader = get_correct_reader(file);
}

fn get_correct_reader(file_path: &Path) -> Box<dyn FastxParser> {
    let mut buf_reader = get_bufreader(file_path);
    let mut first_line = String::new();
    buf_reader
        .read_line(&mut first_line)
        .expect("Could not read from file");

    // Reset the BufReader to the start of the file
    let buf_reader = get_bufreader(file_path);

    if is_fastq(&first_line) {
        Box::new(bio::io::fastq::Reader::new(buf_reader))
    } else if is_fasta(&first_line) {
        Box::new(bio::io::fasta::Reader::new(buf_reader))
    } else {
        panic!("Invalid file format: {file_path:?}");
    }
}

fn get_bufreader(file_path: &Path) -> Box<dyn BufRead> {
    let file =
        std::fs::File::open(file_path).unwrap_or_else(|e| panic!("Failed to open file: {e}"));

    if file_path.extension().take().unwrap() == "gz" {
        let gz = GzDecoder::new(file);
        return Box::new(BufReader::new(gz));
    }

    Box::new(BufReader::new(file))
}

fn is_fastq(first_line: &str) -> bool {
    first_line.chars().next().take().unwrap() == '@'
}

fn is_fasta(first_line: &str) -> bool {
    first_line.chars().next().take().unwrap() == '>'
}
