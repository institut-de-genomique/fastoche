use crate::formatted_metrics::FormattedMetrics;
use crate::metrics::Metrics;
use tabled::object::{Columns, LastRow, Object, Rows};
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

// const OPTIONAL_FIELDS: [&str; 4] = ["ng50", "ng80", "ng90", "mean_quality"];

pub fn print(metrics_vec: &[Metrics]) {
    let fmt = metrics_vec
        .iter()
        .map(FormattedMetrics::from_metrics)
        .collect::<Vec<FormattedMetrics>>();

    let builder = Table::builder(fmt.clone());
    let mut index = builder.index();
    index.transpose();
    let mut table = index.build();
    let mut styled_table = table
        .with(Style::sharp())
        // Left align first column
        .with(Modify::new(Columns::first()).with(Alignment::left()))
        // Rigbt align other columns
        .with(Modify::new(Columns::first().inverse()).with(Alignment::right()))
        // Disable index row
        .with(Disable::row(Rows::first()));

    // Only display NGX and quality if their are greater that 0
    if fmt[0].ng50_lg50 == "0 (0)" {
        styled_table = styled_table.with(Disable::row(Rows::new(12..15)));
    }
    if fmt[0].mean_quality == "0" {
        styled_table = styled_table.with(Disable::row(LastRow));
    }

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
            print!(",{}", &m[f]);
        }
    }

    println!();
}

pub fn print_parsable(
    metrics_vec: &[Metrics],
    user_output_fields: &Option<Vec<String>>,
    no_header: bool,
) {
    // choose the output fields
    let default_output_fields = FIELDS.map(|x| x.to_owned()).to_vec();
    let output_fields = match user_output_fields {
        Some(x) => x,
        None => &default_output_fields,
    };
    // maybe print a header
    if !no_header {
        print!("filename");
        for f in output_fields {
            print!(",{f}");
        }
        println!();
    }
    // print the metrics for this file
    for m in metrics_vec {
        print!("{}", m.filename);
        for f in output_fields {
            print!(",{}", &m[f]);
        }
        println!();
    }
}

pub fn parse_output_format(output_format: &Option<String>) -> Option<Vec<String>> {
    match output_format {
        Some(format_str) => {
            let mut output_fields = Vec::new();
            for field in format_str.split(',') {
                if !FIELDS.contains(&field) {
                    panic!("{field} is not a valid field")
                }
                output_fields.push(field.to_owned());
            }
            if output_fields.is_empty() {
                panic!("Could not parse output format string")
            }
            Some(output_fields)
        }
        None => None,
    }
}
