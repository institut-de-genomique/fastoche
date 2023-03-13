use crate::formatted_metrics::FormattedMetrics;
use crate::metrics::Metrics;
use tabled::object::{Columns, Object, Rows};
use tabled::{Alignment, Disable, Modify, Style, Table};

const FIELDS: [&str; 23] = [
    "cumul",
    "number",
    "min_size",
    "max_size",
    "avg_size",
    "aun",
    "number_n",
    "percent_n",
    "number_gc",
    "percent_gc",
    "n50",
    "l50",
    "n80",
    "l80",
    "n90",
    "l90",
    "ng50",
    "lg50",
    "ng80",
    "lg80",
    "ng90",
    "lg90",
    "mean_quality",
];

pub fn print(metrics_vec: &[Metrics]) {
    let fmt = metrics_vec
        .iter()
        .map(FormattedMetrics::from_metrics)
        .collect::<Vec<FormattedMetrics>>();

    let builder = Table::builder(fmt);
    let mut index = builder.index();
    index.transpose();
    let mut table = index.build();
    let styled_table = table
        .with(Style::sharp())
        // Left align first column
        .with(Modify::new(Columns::first()).with(Alignment::left()))
        // Rigbt align other columns
        .with(Modify::new(Columns::first().inverse()).with(Alignment::right()))
        // Disable index row
        .with(Disable::row(Rows::first()));

    println!("{styled_table}");
}

pub fn print_csv(metrics_vec: &[Metrics]) {
    print!("filename");
    for m in metrics_vec.iter() {
        print!(",{}", m.filename);
    }

    for f in FIELDS {
        print!("\n{f}");
        for m in metrics_vec {
            print!(",{}", m[f]);
        }
    }

    println!();
}

pub fn print_parsable(metrics_vec: &[Metrics]) {
    print!("filename");
    for f in FIELDS {
        print!(",{f}");
    }
    println!();

    for m in metrics_vec {
        print!("{}", m.filename);
        for f in FIELDS {
            print!(",{}", m[f]);
        }
        println!();
    }
}
