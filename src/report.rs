use crate::formatted_metrics::FormattedMetrics;
use crate::metrics::Metrics;
use tabled::object::{Columns, InversionCombination, Object, Rows, Segment};
use tabled::{
    builder, formatting::AlignmentStrategy, Alignment, Disable, Modify, Rotate, Style, Table,
};

pub fn print_report(metrics_vec: &[Metrics]) {
    let fmt = metrics_vec
        .iter()
        .map(FormattedMetrics::from_metrics)
        .collect::<Vec<FormattedMetrics>>();

    let names = metrics_vec
        .iter()
        .map(|metrics| metrics.filename.clone())
        .collect::<Vec<String>>();

    let mut builder = Table::builder(fmt);
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
