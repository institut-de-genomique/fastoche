use crate::metrics::Metrics;
use tabled::Tabled;
use thousands::Separable;

#[derive(Debug, Tabled)]
pub struct FormattedMetrics {
    #[tabled(rename = "")]
    pub basename: String,

    #[tabled(rename = "Cumul. size")]
    pub cumul: String,
    #[tabled(rename = "Seq. number")]
    pub number: String,

    #[tabled(rename = "N50 (L50)")]
    pub n50_l50: String,
    #[tabled(rename = "N80 (L80)")]
    pub n80_l80: String,
    #[tabled(rename = "N90 (L90)")]
    pub n90_l90: String,

    #[tabled(rename = "Min. size")]
    pub min_size: String,
    #[tabled(rename = "Max. size")]
    pub max_size: String,
    #[tabled(rename = "Avg. size")]
    pub avg_size: String,
    #[tabled(rename = "auN")]
    pub aun: String,

    #[tabled(rename = "Ns Number")]
    pub number_n: String,
    #[tabled(rename = "GC Number")]
    pub number_gc: String,
}

impl FormattedMetrics {
    pub fn from_metrics(metrics: &Metrics) -> Self {
        let mut n50_l50 = String::new();
        n50_l50.push_str(&metrics.n50.separate_with_commas());
        n50_l50.push_str(" (");
        n50_l50.push_str(&metrics.l50.separate_with_commas());
        n50_l50.push(')');

        let mut n80_l80 = String::new();
        n80_l80.push_str(&metrics.n80.separate_with_commas());
        n80_l80.push_str(" (");
        n80_l80.push_str(&metrics.l80.separate_with_commas());
        n80_l80.push(')');

        let mut n90_l90 = String::new();
        n90_l90.push_str(&metrics.n90.separate_with_commas());
        n90_l90.push_str(" (");
        n90_l90.push_str(&metrics.l90.separate_with_commas());
        n90_l90.push(')');

        Self {
            basename: metrics.filename.clone(),
            cumul: metrics.cumul.separate_with_commas(),
            number: metrics.number.separate_with_commas(),
            min_size: metrics.min_size.separate_with_commas(),
            max_size: metrics.max_size.separate_with_commas(),
            avg_size: metrics.avg_size.separate_with_commas(),
            aun: metrics.aun.separate_with_commas(),
            number_n: metrics.number_n.separate_with_commas(),
            number_gc: metrics.number_gc.separate_with_commas(),
            n50_l50,
            n80_l80,
            n90_l90,
        }
    }
}
