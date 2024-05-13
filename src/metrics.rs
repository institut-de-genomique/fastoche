use std::{fmt::Display, ops::Index};

#[derive(Debug)]
pub struct Metrics {
    pub filename: String,
    pub genome_size: i64,
    pub cumul: usize,
    pub number: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub avg_size: usize,
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
    pub seq_sizes: Vec<usize>,
    pub nucleotide_counts: [usize; 256],
    pub mean_qualities: Vec<f64>,
    pub mean_quality: usize,
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
            seq_sizes: Vec::new(),
            nucleotide_counts: [0; 256],
            mean_qualities: Vec::new(),
            mean_quality: 0,
        }
    }

    pub fn compute(&mut self) {
        self.seq_sizes.sort_by(|a, b| b.cmp(a));

        self.compute_seq_number();
        self.compute_cumul();
        self.compute_min_size();
        self.compute_max_size();
        self.compute_avg_size();
        self.compute_number_n();
        self.compute_number_gc();
        self.compute_aun_and_nx_metrics();

        self.seq_sizes = Vec::new();

        self.compute_mean_quality();
    }

    fn compute_seq_number(&mut self) {
        self.number = self.seq_sizes.len();
    }

    fn compute_cumul(&mut self) {
        for size in &self.seq_sizes {
            self.cumul += *size;
        }
    }

    fn compute_min_size(&mut self) {
        self.min_size = *self.seq_sizes.last().expect("Failed to get first element");
    }

    fn compute_max_size(&mut self) {
        self.max_size = *self.seq_sizes.first().expect("Failed to get last element");
    }

    fn compute_avg_size(&mut self) {
        self.avg_size = self.seq_sizes.iter().sum::<usize>() / self.number;
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
        let breakpoints: Vec<usize> = vec![
            (0.5 * self.cumul as f64) as usize,
            (0.8 * self.cumul as f64) as usize,
            (0.9 * self.cumul as f64) as usize,
            (1.1 * self.cumul as f64) as usize,
        ];
        let breakpoints_g: Vec<usize> = vec![
            (0.5 * self.genome_size as f64) as usize,
            (0.8 * self.genome_size as f64) as usize,
            (0.9 * self.genome_size as f64) as usize,
            (1000_f64 * self.genome_size as f64) as usize,
        ];
        let mut current_breakpoint: usize = 0;
        let mut current_breakpoint_g: usize = 0;
        let mut current_lx = 0;
        let mut current_lx_g = 0;
        let mut cumul: usize = 0;

        for size in &self.seq_sizes {
            cumul += *size;
            current_lx += 1;
            current_lx_g += 1;
            self.aun += f64::powi(*size as f64, 2_i32) as usize;

            if cumul >= breakpoints[current_breakpoint] {
                match current_breakpoint {
                    0 => {
                        self.n50 = *size;
                        self.l50 = current_lx;
                    }
                    1 => {
                        self.n80 = *size;
                        self.l80 = current_lx;
                    }
                    2 => {
                        self.n90 = *size;
                        self.l90 = current_lx;
                    }
                    _ => {}
                }

                current_breakpoint += 1;
            }

            if self.genome_size > 0 && cumul >= breakpoints_g[current_breakpoint_g] {
                match current_breakpoint_g {
                    0 => {
                        self.ng50 = *size;
                        self.lg50 = current_lx_g;
                    }
                    1 => {
                        self.ng80 = *size;
                        self.lg80 = current_lx_g;
                    }
                    2 => {
                        self.ng90 = *size;
                        self.lg90 = current_lx_g;
                    }
                    _ => {}
                }

                current_breakpoint_g += 1;
            }
        }

        self.aun = (self.aun as f64 / self.cumul as f64) as usize;
    }

    fn compute_mean_quality(&mut self) {
        let mut mean_quality: f64 = 0.0;
        for q in &self.mean_qualities {
            mean_quality += q;
        }
        mean_quality /= self.mean_qualities.len() as f64;
        self.mean_quality = mean_quality as usize;

        self.mean_qualities = Vec::new();
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
            _ => panic!("Unknown field: {index}"),
        }
    }
}
