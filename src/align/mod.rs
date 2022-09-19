//! # Processing SAM, BAM, and CRAM alignment files
//! Functions and methods related to processing alignment files, such as [SAM, BAM](https://samtools.github.io/hts-specs/SAMv1.pdf), and [CRAM](https://samtools.github.io/hts-specs/CRAMv3.pdf) files.

pub mod filter;
pub mod info_stats;
pub mod reader;

use self::{info_stats::SamBamCramStats, reader::SamBamCramReader};
use crate::{
    cli::CliOpt,
    record::stats::RecordStats,
    utils::{formats::OutputFormat, Align, Hts, HtsFile},
};
use bam::{BamReader, SamReader};
use clap::Parser;
use std::path::PathBuf;

/// CLI options for getting info from an HTS file
#[derive(Debug, Parser)]
pub(crate) struct SamBamCramInfoOpts {
    /// Get info about this HTS file
    #[clap(name = "HTS")]
    hts_path: PathBuf,

    /// Count the total number of records
    #[clap(short, long)]
    total: bool,

    /// Track the frequency of sequence lengths
    #[clap(short, long)]
    lengths: bool,

    /// Track the sequencing instruments used
    #[clap(short, long)]
    instruments: bool,

    /// Track flow cell IDs
    #[clap(short = 'F', long)]
    flow_cell_ids: bool,

    /// Output format to return statistics in
    #[clap(short = 'f', long, default_value = "human")]
    format: OutputFormat,

    /// Keep statistics on the first N records
    #[clap(short = 'N', long = "max-records", name = "N")]
    n_max_records: Option<u64>,
}

impl SamBamCramInfoOpts {
    /// Get information and statistics about a desired FASTQ file
    fn calc_info(&self, hts: HtsFile) -> SamBamCramStats {
        let mut stats = SamBamCramStats::new();
        let reader_wrapper = match hts.filetype() {
            Hts::Align(Align::Sam) => SamBamCramReader::Sam(
                SamReader::from_path(hts.path()).expect("Error opening SAM file."),
            ),
            Hts::Align(Align::Bam) => SamBamCramReader::Bam(
                BamReader::from_path(hts.path(), 3).expect("Error opening BAM file."),
            ),
            Hts::Align(Align::Cram) => {
                todo!()
            }
            _ => todo!(),
        };

        match reader_wrapper {
            SamBamCramReader::Bam(mut reader) => {
                if let Some(n_max) = self.n_max_records {
                    // check if the max capacity has been hit
                    while let (true, Some(record)) = (stats.n_records() < n_max, reader.next()) {
                        stats.process_record(&record, self);
                    }
                } else {
                    while let Some(record) = reader.next() {
                        stats.process_record(&record, self);
                    }
                }
            }
            SamBamCramReader::Sam(mut reader) => {
                if let Some(n_max) = self.n_max_records {
                    // check if the max capacity has been hit
                    while let (true, Some(record)) = (stats.n_records() < n_max, reader.next()) {
                        stats.process_record(&record, self);
                    }
                } else {
                    while let Some(record) = reader.next() {
                        stats.process_record(&record, self);
                    }
                }
            }
            SamBamCramReader::Cram => {
                todo!()
            }
        }

        stats
    }
}

impl CliOpt for SamBamCramInfoOpts {
    fn exec(&self) {
        let hts = HtsFile::new(&self.hts_path);
        let stats = self.calc_info(hts);
        println!("{:#?}", stats);
    }
}
