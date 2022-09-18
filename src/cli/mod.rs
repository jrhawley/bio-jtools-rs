//! Command line interface options and parsing

use crate::fastq::FastqInfoOpts;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub(crate) cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SubCmd {
    #[clap(subcommand)]
    Info(InfoSubCmd),

    /// Filter an HTS file by its query names
    Filter,

    /// Organize a batch of raw sequencing data
    #[clap(name = "org")]
    Organize,
}

pub(crate) trait CliOpt {
    fn exec(&self);
}

/// Collect information and calculate statistics about an HTS file
#[derive(Debug, Subcommand)]
pub(crate) enum InfoSubCmd {
    /// Get info about a FASTA file
    #[clap(visible_alias = "fa")]
    Fasta,

    /// Get info about a FASTQ file
    #[clap(visible_alias = "fq")]
    Fastq(FastqInfoOpts),

    /// Get info about a SAM/BAM/CRAM file
    #[clap(visible_aliases = &["cram", "sam"])]
    Bam,

    /// Get info about a BED file
    Bed,
}
