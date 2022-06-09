//! Command line interface options and parsing

use clap::{Parser, Subcommand};

use crate::fastx::InfoOpts;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub(crate) cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SubCmd {
    Info(InfoOpts),

    /// Filter an HTS file by its query names
    Filter,

    /// Organize a batch of raw sequencing data
    #[clap(name = "org")]
    Organize,
}

pub(crate) trait CliOpt {
    fn exec(&self);
}
