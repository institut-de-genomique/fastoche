use crate::metrics::Metrics;
use crate::report::print_report;
use flate2::read::GzDecoder;
use std::path::{Path, PathBuf};

pub fn parse(files: &[PathBuf], min_size: usize, parsable: bool) {
    let mut metrics_vec = Vec::new();
    for f in files.iter() {
        metrics_vec.push(compute_stats(f, min_size));
    }

    if !parsable {
        print_report(&metrics_vec);
    }
}

fn compute_stats(file_path: &Path, min_size: usize) -> Metrics {
    let mut reader = get_reader(file_path);
    let mut metrics = Metrics::new(&file_path.to_str().unwrap().to_string());

    while let Some(record) = reader.next() {
        let record = record.expect("Error");
        let record_len = record.seq().len();

        if record_len < min_size {
            continue;
        }

        metrics.seq_sizes.push(record_len);
        for c in record.seq().iter() {
            metrics.nucleotide_counts[*c as usize] += 1;
        }
    }

    metrics.compute();
    metrics
}

fn get_reader(file_path: &Path) -> Box<dyn needletail::FastxReader> {
    assert!(file_path.exists(), "File not found {file_path:?}");

    let file =
        std::fs::File::open(file_path).unwrap_or_else(|e| panic!("Failed to open file: {e}"));

    let reader = if file_path.extension().take().unwrap() == "gz" {
        let gz = GzDecoder::new(file);
        needletail::parse_fastx_reader(gz).unwrap()
    } else {
        needletail::parse_fastx_reader(file).unwrap()
    };

    reader
}
