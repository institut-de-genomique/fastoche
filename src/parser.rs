use crate::metrics::Metrics;
use flate2::read::GzDecoder;
use std::path::{Path, PathBuf};
use tabled::{builder, object::Rows, Disable, Rotate, Style, Table};

pub fn parse(files: &[PathBuf], min_size: usize) {
    for f in files.iter() {
        compute_stats(f, min_size);
    }
}

fn compute_stats(file_path: &Path, min_size: usize) {
    let mut reader = get_reader(file_path);
    let mut metrics = Metrics::new();

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

    // let table = Table::new(vec![metrics])
    //     .with(Disable::row(Rows::first()))
    //     .with(Rotate::Left)
    //     // .with(Rotate::Top)
    //     .with(Style::empty())
    //     .to_string();

    let mut builder = Table::builder(vec![metrics]);
    let mut index = builder.index();
    index.transpose();

    let mut table = index.build();
    let style = Style::modern()
        .off_horizontal()
        .off_vertical()
        .horizontals([
            tabled::style::HorizontalLine::new(1, Style::modern().get_horizontal())
                .main(Some('═'))
                .intersection(None),
        ])
        .verticals([tabled::style::VerticalLine::new(
            1,
            Style::modern().get_vertical(),
        )]);
    let styled_table = table.with(style);

    println!("{styled_table}");
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
