use crate::report::print;
use crate::{metrics::Metrics, report::print_csv, report::print_parsable};
use flate2::read::GzDecoder;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn parse(
    files: &[PathBuf],
    min_size: usize,
    genome_size: i64,
    qual_offset: u8,
    parsable: bool,
    csv: bool,
    per_seq: Option<PathBuf>,
) {
    let mut per_seq_writer: Option<BufWriter<std::fs::File>> = None;
    if let Some(path) = per_seq {
        let file =
            std::fs::File::create(path).unwrap_or_else(|e| panic!("Failed to create file: {e}"));
        per_seq_writer = Some(BufWriter::new(file));
    }

    let mut metrics_vec = Vec::new();
    for f in files.iter() {
        metrics_vec.push(compute_stats(
            f,
            min_size,
            genome_size,
            qual_offset,
            &mut per_seq_writer,
        ));
    }

    if csv {
        print_csv(&metrics_vec);
    } else if parsable {
        print_parsable(&metrics_vec);
    } else {
        print(&metrics_vec);
    }
}

fn compute_stats(
    file_path: &Path,
    min_size: usize,
    genome_size: i64,
    qual_offset: u8,
    per_seq_writer: &mut Option<BufWriter<std::fs::File>>,
) -> Metrics {
    let mut reader = get_reader(file_path);
    let mut metrics = Metrics::new(file_path.to_str().unwrap(), genome_size);

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

        if let Some(qualities) = record.qual() {
            let mut avg_quality: f64 = 0_f64;
            for q in qualities {
                avg_quality += (q - qual_offset) as f64;
            }

            if let Some(writer) = per_seq_writer {
                let record_id = std::str::from_utf8(record.id()).unwrap();

                let record_gc = record
                    .seq()
                    .iter()
                    .filter(|c| **c == b'G' || **c == b'C')
                    .count()
                    * 100;

                write!(
                    writer,
                    "{}\t{}\t{}\t{}\n",
                    record_id,
                    record_len,
                    &format!("{:.2}", record_gc as f64 / record_len as f64),
                    &format!("{:.2}", avg_quality as f64 / record_len as f64),
                )
                .unwrap();
            }

            metrics
                .mean_qualities
                .push(avg_quality as f64 / record_len as f64);
        }
    }

    metrics.compute();
    metrics
}

fn get_reader(file_path: &Path) -> Box<dyn needletail::FastxReader> {
    assert!(file_path.exists(), "File not found {file_path:?}");

    let file =
        std::fs::File::open(file_path).unwrap_or_else(|e| panic!("Failed to open file: {e}"));
    let buf_reader = std::io::BufReader::new(file);

    let reader = if file_path.extension().take().unwrap_or_else(|| panic!("File extension should not be empty! As an example, file should be named 'toto.fasta' and not 'toto'.")) == "gz" {
        let gz = GzDecoder::new(buf_reader);
        needletail::parse_fastx_reader(gz).unwrap()
    } else {
        needletail::parse_fastx_reader(buf_reader).unwrap()
    };

    reader
}
