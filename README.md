Fastoche is a tool designed to effortlessly extract common metrics from Fasta or Fastq files, gzipped or not.

# Installation
## Compiling from source
The only dependency to build the code from source is the [Rust](https://www.rust-lang.org/fr/tools/install) compiler.
```bash
git clone https://github.com/institut-de-genomique/fastoche
cd fastoche
cargo install --path . --root .
```
If everything went as expected, the `fastoche` binary file will be in the `bin` folder.

# Usage
You can run `fastoche -h` to get a detailed view of `fastoche` options. The basic usage would be to give some fasta/fastq files and rename the columns to get customized table headers:
```fastoche
fastoche -f reads.fastq.gz -f ../assembly.fasta -r "Reads,Assembly"

┌──────────────┬────────────────────┬──────────────────────┐
│              │              Reads │             Assembly │
├──────────────┼────────────────────┼──────────────────────┤
│ Cumul. size  │          5,957,360 │        2,275,546,859 │
│ Seq. number  │              1,000 │               33,632 │
│ N50 (L50)    │        8,383 (229) │      119,535 (5,017) │
│ N80 (L80)    │        4,170 (530) │      43,439 (14,548) │
│ N90 (L90)    │        3,016 (697) │      27,849 (21,103) │
│ Min. size    │                159 │                3,489 │
│ Max. size    │             28,705 │            1,362,978 │
│ Avg. size    │              5,957 │               67,660 │
│ auN          │              9,598 │              176,634 │
│ Ns Number    │          0 (0.00%) │            0 (0.00%) │
│ GC Number    │ 2,542,770 (42.68%) │ 995,607,282 (43.75%) │
│ Mean quality │                  9 │                    0 │
└──────────────┴────────────────────┴──────────────────────┘
```
The `-f` flag can be given as many times as needed and can contain a mix of fasta and fastq files. The `Mean quality` row is shown only for Fastq files and is always equal to 0 for Fasta files, as Phred qualities are not available.
 


