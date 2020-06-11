# bio-jtools-rs
A suite of bioinformatics tools for interacting with high throughput sequencing (HTS) data, written entirely in Rust

![Crates.io](https://img.shields.io/crates/v/bio-jtools)

## Suite

### info

Extract and print metadata about an HTS file.
For FASTQs, this includes number of bases, number of records, and all the instruments the records come from.

### filter

Filter an HTS file by its query names.
**Not currently implemented**.

### jaccard

Calculate the Jaccard index for each pair in a set of BED files.
**Not currently implemented**.

### kspec

Calculate the k-mer spectra of an HTS file.
**Not currently implemented**.

### org

Organize a batch of raw sequencing data.

This takes a folder directly from an Illumina sequencer with FASTQ files and organizes them as follows, ready for alginment and quality control:

```shell
YYMMDD_INSTID_RUN_FCID/
├── FASTQs/                     # home for your raw data
    ├── Sample1_R1.fastq.gz
    ├── Sample1_R2.fastq.gz
    └── ...
├── Aligned/                    # a home for your aligned data
├── Reports/                    # QC reports, etc files
├── config.tsv                  # a table of samples (rows) x features (cols)
├── cluster.yaml                # a yaml file of cluster parameters for jobs in the Snakefile
├── README.md                   # description of the folder, data contents
├── setup.log                   # a log of what operations were performed with `bjt org`
└── Snakefile                   # Snakemake workflow file
```

## Benchmarking

Benchmarks on run in a Windows 10 computer, Intel i5 750 @ 2.67 GHz processor with 12 GB of RAM.
Times are listed +/- standard deviation, using `hyperfine --warmup 2`.

### info

| File | # Reads | t<sub>gzip</sub> | t<sub>plain</sub> |
| ---------------------- | ------- | -------------------- | --------------------- |
| [`examples/SRR0000001.fastq`](examples/) | 2 500  | 17.0 ms +/- 0.9 ms | 12.8 ms +/- 1.0 ms |
| [`examples/SRR0000002.fastq`](examples/) | 25 000 | 78.4 ms +/- 3.3 ms | 16.6 ms +/- 0.6 ms |
| [`M_abscessus_HiSeq.fq`](https://lh3.github.io/2020/05/17/fast-high-level-programming-languages) | 5 682 010 | 12.323 s +/- 0.227 s | 951.5 ms + 5.6 ms |
