use std::{collections::BTreeMap, fmt::Display, ops::Index};

#[derive(Debug)]
pub struct Metrics {
    pub filename: String,
    pub genome_size: i64,
    pub cumul: usize,
    pub number: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub avg_size: usize,
    pub med_size: usize,
    pub aun: usize,
    pub number_n: usize,
    pub percent_n: f64,
    pub number_gc: usize,
    pub percent_gc: f64,
    pub n50: usize,
    pub l50: usize,
    pub n80: usize,
    pub l80: usize,
    pub n90: usize,
    pub l90: usize,
    pub ng50: usize,
    pub lg50: usize,
    pub ng80: usize,
    pub lg80: usize,
    pub ng90: usize,
    pub lg90: usize,
    length_counts: BTreeMap<usize, usize>,
    pub nucleotide_counts: [usize; 256],
    quality_counts: BTreeMap<u64, usize>,
    pub mean_quality: usize,
    pub median_quality: usize,
}

impl Metrics {
    pub fn new(filename: &str, genome_size: i64, name: Option<String>) -> Self {
        let basename = match name {
            Some(n) => n,
            None => filename
                .split('/')
                .last()
                .expect("Could not get last element")
                .replace(".fasta", "")
                .replace(".fastq", "")
                .replace(".fa", "")
                .replace(".fq", "")
                .replace(".gz", ""),
        };

        Metrics {
            filename: basename,
            genome_size,
            cumul: 0,
            number: 0,
            min_size: 0,
            max_size: 0,
            avg_size: 0,
            med_size: 0,
            aun: 0,
            number_n: 0,
            percent_n: 0.0,
            number_gc: 0,
            percent_gc: 0.0,
            n50: 0,
            l50: 0,
            n80: 0,
            l80: 0,
            n90: 0,
            l90: 0,
            ng50: 0,
            lg50: 0,
            ng80: 0,
            lg80: 0,
            ng90: 0,
            lg90: 0,
            length_counts: BTreeMap::new(),
            nucleotide_counts: [0; 256],
            quality_counts: BTreeMap::new(),
            mean_quality: 0,
            median_quality: 0,
        }
    }

    pub fn add_length(&mut self, length: usize) {
        *self.length_counts.entry(length).or_default() += 1;
    }

    pub fn add_quality(&mut self, mean_quality: f64) {
        *self
            .quality_counts
            .entry(mean_quality.to_bits())
            .or_default() += 1;
    }

    pub fn compute(&mut self) {
        self.compute_seq_number();
        if self.number == 0 {
            return;
        }
        self.compute_cumul();
        self.compute_min_size();
        self.compute_max_size();
        self.compute_avg_size();
        self.compute_med_size();
        if self.cumul > 0 {
            self.compute_number_n();
            self.compute_number_gc();
            self.compute_aun_and_nx_metrics();
        }

        self.length_counts.clear();

        self.compute_quality_metrics();
        self.quality_counts.clear();
    }

    fn compute_seq_number(&mut self) {
        self.number = self.length_counts.values().sum();
    }

    fn compute_cumul(&mut self) {
        self.cumul = self
            .length_counts
            .iter()
            .map(|(size, count)| size * count)
            .sum();
    }

    fn compute_min_size(&mut self) {
        self.min_size = *self.length_counts.first_key_value().unwrap().0;
    }

    fn compute_max_size(&mut self) {
        self.max_size = *self.length_counts.last_key_value().unwrap().0;
    }

    fn compute_avg_size(&mut self) {
        self.avg_size = self.cumul / self.number;
    }

    fn compute_med_size(&mut self) {
        let mut rank = (self.number - 1) / 2;
        for (&size, &count) in &self.length_counts {
            if rank < count {
                self.med_size = size;
                break;
            }
            rank -= count;
        }
    }

    fn compute_number_n(&mut self) {
        self.number_n =
            self.nucleotide_counts[b'N' as usize] + self.nucleotide_counts[b'n' as usize];
        self.percent_n = (self.number_n as f64 / self.cumul as f64) * 100.0;
    }

    fn compute_number_gc(&mut self) {
        self.number_gc = self.nucleotide_counts[b'G' as usize]
            + self.nucleotide_counts[b'C' as usize]
            + self.nucleotide_counts[b'g' as usize]
            + self.nucleotide_counts[b'c' as usize];
        self.percent_gc = (self.number_gc as f64 / self.cumul as f64) * 100.0;
    }

    fn compute_aun_and_nx_metrics(&mut self) {
        let sum_squares: u128 = self
            .length_counts
            .iter()
            .map(|(&size, &count)| size as u128 * size as u128 * count as u128)
            .sum();
        self.aun = (sum_squares / self.cumul as u128) as usize;

        [
            (self.n50, self.l50),
            (self.n80, self.l80),
            (self.n90, self.l90),
        ] = self.nx_metrics(self.cumul);
        if self.genome_size > 0 {
            [
                (self.ng50, self.lg50),
                (self.ng80, self.lg80),
                (self.ng90, self.lg90),
            ] = self.nx_metrics(self.genome_size as usize);
        }
    }

    fn nx_metrics(&self, total: usize) -> [(usize, usize); 3] {
        let thresholds = [0.5, 0.8, 0.9].map(|fraction| (fraction * total as f64) as usize);
        let mut result = [(0, 0); 3];
        let mut next = 0;
        let mut bases = 0;
        let mut reads = 0;

        for (&size, &count) in self.length_counts.iter().rev() {
            if size == 0 {
                break;
            }
            let end = bases + size * count;
            while next < thresholds.len() && thresholds[next] <= end {
                // A threshold may fall partway through a group of equal lengths.
                let remaining = thresholds[next].saturating_sub(bases);
                let within_group = 1 + remaining.saturating_sub(1) / size;
                result[next] = (size, reads + within_group);
                next += 1;
            }
            if next == thresholds.len() {
                break;
            }
            bases = end;
            reads += count;
        }

        result
    }

    fn compute_quality_metrics(&mut self) {
        let number: usize = self.quality_counts.values().sum();
        if number == 0 {
            return;
        }

        let lower_rank = (number - 1) / 2;
        let upper_rank = number / 2;
        let mut seen = 0;
        let mut lower = 0.0;
        let mut upper = 0.0;
        let mut sum = 0.0;

        for (&bits, &count) in &self.quality_counts {
            let quality = f64::from_bits(bits);
            let end = seen + count;
            if seen <= lower_rank && lower_rank < end {
                lower = quality;
            }
            if seen <= upper_rank && upper_rank < end {
                upper = quality;
            }
            for _ in 0..count {
                sum += quality;
            }
            seen = end;
        }

        self.mean_quality = (sum / number as f64) as usize;
        self.median_quality = ((lower + upper) / 2.0) as usize;
    }
}

pub trait Num: Display {}
impl Num for usize {}
impl Num for f64 {}

impl Index<&str> for Metrics {
    type Output = dyn Num;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "cumul" => &self.cumul,
            "number" => &self.number,
            "min_size" => &self.min_size,
            "max_size" => &self.max_size,
            "avg_size" => &self.avg_size,
            "med_size" => &self.med_size,
            "aun" => &self.aun,
            "number_n" => &self.number_n,
            "percent_n" => &self.percent_n,
            "number_gc" => &self.number_gc,
            "percent_gc" => &self.percent_gc,
            "n50" => &self.n50,
            "l50" => &self.l50,
            "n80" => &self.n80,
            "l80" => &self.l80,
            "n90" => &self.n90,
            "l90" => &self.l90,
            "ng50" => &self.ng50,
            "lg50" => &self.lg50,
            "ng80" => &self.ng80,
            "lg80" => &self.lg80,
            "ng90" => &self.ng90,
            "lg90" => &self.lg90,
            "mean_quality" => &self.mean_quality,
            "median_quality" => &self.median_quality,
            _ => panic!("Unknown field: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Metrics;

    fn length_metrics(lengths: &[usize], genome_size: i64) -> Metrics {
        let mut metrics = Metrics::new("lengths.fasta", genome_size, None);
        for &length in lengths {
            metrics.add_length(length);
        }
        metrics.compute();
        metrics
    }

    fn quality_metrics(qualities: &[f64]) -> Metrics {
        let mut metrics = Metrics::new("qualities.fastq", 0, None);
        for &quality in qualities {
            metrics.add_length(10);
            metrics.add_quality(quality);
        }
        metrics.compute();
        metrics
    }

    #[test]
    fn test_thresholds_inside_repeated_length_groups() {
        let metrics = length_metrics(&[200, 150, 150, 150, 150, 100, 100], 1200);
        assert_eq!((metrics.number, metrics.cumul), (7, 1000));
        assert_eq!((metrics.min_size, metrics.max_size), (100, 200));
        assert_eq!(
            (metrics.avg_size, metrics.med_size, metrics.aun),
            (142, 150, 150)
        );
        assert_eq!((metrics.n50, metrics.l50), (150, 3));
        assert_eq!((metrics.n80, metrics.l80), (150, 5));
        assert_eq!((metrics.n90, metrics.l90), (100, 6));
        assert_eq!((metrics.ng50, metrics.lg50), (150, 4));
        assert_eq!((metrics.ng80, metrics.lg80), (100, 7));
        assert_eq!((metrics.ng90, metrics.lg90), (0, 0));
    }

    #[test]
    fn test_one_sequence_can_cross_multiple_thresholds() {
        let metrics = length_metrics(&[1000, 1, 1], 1000);
        assert_eq!((metrics.n50, metrics.l50), (1000, 1));
        assert_eq!((metrics.n80, metrics.l80), (1000, 1));
        assert_eq!((metrics.n90, metrics.l90), (1000, 1));
        assert_eq!((metrics.ng50, metrics.lg50), (1000, 1));
        assert_eq!((metrics.ng80, metrics.lg80), (1000, 1));
        assert_eq!((metrics.ng90, metrics.lg90), (1000, 1));
    }

    #[test]
    fn test_even_length_median_uses_lower_middle() {
        let metrics = length_metrics(&[400, 200, 100, 100], 0);
        assert_eq!(metrics.med_size, 100);
    }

    #[test]
    fn test_quality_median_preserves_fractional_middle_values() {
        let metrics = quality_metrics(&[11.9, 10.9, 11.9, 10.9]);
        assert_eq!(metrics.mean_quality, 11);
        assert_eq!(metrics.median_quality, 11);
    }

    #[test]
    fn test_quality_statistics_weight_repeated_values() {
        let metrics = quality_metrics(&[30.0, 11.9, 10.9, 11.9, 11.9]);
        assert_eq!(metrics.mean_quality, 15);
        assert_eq!(metrics.median_quality, 11);
    }

    #[test]
    fn test_quality_mean_preserves_integer_boundary() {
        // Means from two real 150-base reads, repeated 63 and 2 times.
        // Multiplying each mean by its count rounds the result below 39.
        let mut qualities = [5890.0 / 150.0; 65];
        qualities[63..].fill(4590.0 / 150.0);
        let metrics = quality_metrics(&qualities);
        assert_eq!(metrics.mean_quality, 39);
    }

    #[test]
    fn test_quality_mean_is_independent_of_input_order() {
        // Descending input would truncate to 35 if summed before sorting.
        let mut qualities = [5547.0 / 150.0; 113];
        qualities[64..].fill(5208.0 / 150.0);
        let descending = quality_metrics(&qualities);
        qualities.reverse();
        let ascending = quality_metrics(&qualities);
        assert_eq!(descending.mean_quality, 36);
        assert_eq!(ascending.mean_quality, 36);
    }
}
