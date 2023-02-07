# bio-jtools-rs

A suite of bioinformatics tools for interacting with high throughput sequencing (HTS) data, written entirely in Rust

![Crates.io](https://img.shields.io/crates/v/bio-jtools)

## Suite

### info

Extract and print metadata about an HTS file.
For FASTQs, this includes number of bases, number of records, and all the instruments the records come from.

### filter

Filter an HTS file by its query names.
Currently only implemented for SAM/BAM files

### jaccard

Calculate the Jaccard index for each pair in a set of BED files.
Can save the results in a comma-separated file, if specified.

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
