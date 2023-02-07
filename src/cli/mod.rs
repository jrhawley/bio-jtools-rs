//! Command line interface options and parsing

use crate::{
    align::info_stats::SamBamCramInfoOpts,
    fastq::info_stats::FastqInfoOpts,
    // align::{filter::SamBamCramFilterOpts, info_stats::SamBamCramInfoOpts},
    // fastq::{filter::FastqFilterOpts, info_stats::FastqInfoOpts},
};
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

    #[clap(subcommand)]
    Filter(FilterSubCmd),

    /// Organize a batch of raw sequencing data
    #[clap(name = "org")]
    Organize,
}

pub(crate) trait CliOpt {
    fn exec(&self) -> anyhow::Result<()> {
        Ok(())
    }
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
    #[clap(visible_aliases = &["sam", "cram"])]
    Bam(SamBamCramInfoOpts),

    /// Get info about a BED file
    Bed,
}

impl CliOpt for InfoSubCmd {
    fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Fasta => todo!(),
            Self::Fastq(opts) => opts.exec(),
            Self::Bam(opts) => opts.exec(),
            Self::Bed => todo!(),
        }
    }
}

/// Filter an HTS file by its records' properties
#[derive(Debug, Subcommand)]
pub(crate) enum FilterSubCmd {
    /// Filter a FASTA file
    #[clap(visible_alias = "fa")]
    Fasta,

    // /// Filter a FASTQ file
    // #[clap(visible_alias = "fq")]
    // Fastq(FastqFilterOpts),

    // /// Filter a SAM/BAM/CRAM file
    // #[clap(visible_aliases = &["sam", "cram"])]
    // Bam(SamBamCramFilterOpts),

    /// Filter a BED file
    Bed,
}

impl CliOpt for FilterSubCmd {
    fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Fasta => todo!(),
            // Self::Fastq(opts) => opts.exec(),
            // Self::Bam(opts) => opts.exec(),
            Self::Bed => todo!(),
        }
    }
}
