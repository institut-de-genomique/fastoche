use crate::report::print;
use crate::{metrics::Metrics, report::print_csv, report::print_parsable};
use flate2::read::GzDecoder;
use needletail::parser::SequenceRecord;
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
    rename: Option<String>,
    output_fields: Option<Vec<String>>,
    no_header: bool,
) {
    let mut per_seq_writer: Option<BufWriter<std::fs::File>> = None;
    if let Some(path) = per_seq {
        let file =
            std::fs::File::create(path).unwrap_or_else(|e| panic!("Failed to create file: {e}"));
        per_seq_writer = Some(BufWriter::new(file));
    }

    let splits = rename.map(|names| {
        names
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    });

    let mut metrics_vec = Vec::new();
    for (i, f) in files.iter().enumerate() {
        let name = splits.as_ref().map(|names| names[i].clone());

        metrics_vec.push(compute_stats(
            f,
            min_size,
            genome_size,
            qual_offset,
            &mut per_seq_writer,
            name,
        ));
    }

    if csv {
        print_csv(&metrics_vec);
    } else if parsable {
        print_parsable(&metrics_vec, &output_fields, no_header);
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
    name: Option<String>,
) -> Metrics {
    let mut reader = get_reader(file_path);
    let mut metrics = Metrics::new(file_path.to_str().unwrap(), genome_size, name);

    while let Some(record) = reader.next() {
        let record = record.expect("Error");
        let record_len = record.seq().len();

        if record_len < min_size {
            continue;
        }

        metrics.seq_sizes.push(record_len);
        count_nucleotides(&mut metrics, &record.seq());
        let avg_quality = compute_avg_quality(&mut metrics, record.qual(), qual_offset);
        write_per_seq(record, per_seq_writer, avg_quality, record_len);
    }

    metrics.compute();
    metrics
}

fn count_nucleotides(metrics: &mut Metrics, seq: &[u8]) {
    for c in seq.iter() {
        metrics.nucleotide_counts[*c as usize] += 1;
    }
}

fn compute_avg_quality(metrics: &mut Metrics, qualities: Option<&[u8]>, qual_offset: u8) -> f64 {
    let mut avg_quality: f64 = 0_f64;

    if let Some(qualities) = qualities {
        for q in qualities {
            avg_quality += (q - qual_offset) as f64;
        }
        metrics
            .mean_qualities
            .push(avg_quality / qualities.len() as f64);
    }

    avg_quality
}

fn write_per_seq(
    record: SequenceRecord,
    writer: &mut Option<BufWriter<std::fs::File>>,
    avg_quality: f64,
    record_len: usize,
) {
    if let Some(writer) = writer {
        let record_id = std::str::from_utf8(record.id()).unwrap();

        let record_gc = record
            .seq()
            .iter()
            .filter(|c| **c == b'G' || **c == b'C')
            .count()
            * 100;

        writeln!(
            writer,
            "{}\t{}\t{}\t{}",
            record_id,
            record_len,
            &format!("{:.2}", record_gc as f64 / record_len as f64),
            &format!("{:.2}", avg_quality / record_len as f64),
        )
        .unwrap();
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_reads_metrics() -> Metrics {
        let path = std::path::Path::new("test_inputs/reads.fastq.gz");

        let mut per_seq_writer = None;

        compute_stats(path, 0, 0, 33, &mut per_seq_writer, None)
    }

    #[test]
    fn test_reads_cumul() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.cumul, 5957360);
    }

    #[test]
    fn test_reads_number() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.number, 1000);
    }

    #[test]
    fn test_reads_n50() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.n50, 8383);
    }

    #[test]
    fn test_reads_l50() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.l50, 229);
    }

    #[test]
    fn test_reads_n80() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.n80, 4170);
    }

    #[test]
    fn test_reads_l80() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.l80, 530);
    }

    #[test]
    fn test_reads_n90() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.n90, 3016);
    }

    #[test]
    fn test_reads_l90() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.l90, 697);
    }

    #[test]
    fn test_reads_aun() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.aun, 9598);
    }

    #[test]
    fn test_reads_min_size() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.min_size, 159);
    }

    #[test]
    fn test_reads_avg_size() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.avg_size, 5957);
    }

    #[test]
    fn test_reads_max_size() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.max_size, 28705);
    }

    #[test]
    fn test_reads_number_n() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.number_n, 0);
    }

    #[test]
    fn test_reads_number_gc() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.number_gc, 2542770);
    }

    #[test]
    fn test_reads_mean_quality() {
        let metrics = setup_reads_metrics();
        assert_eq!(metrics.mean_quality, 9);
    }
}
