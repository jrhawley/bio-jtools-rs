//! # Output formats
//!
//! Handle the formats in which the data can be returned.

use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputFormatError {
    #[error("Output format {0} not understood.")]
    UnknownFormat(String),
}

#[derive(Debug)]
pub enum OutputFormat {
    HumanReadable,
    Csv,
    Json,
    Toml,
    Tsv,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = OutputFormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "h" | "H" | "human" | "Human" => Ok(OutputFormat::HumanReadable),
            "c" | "C" | "csv" | "Csv" | "CSV" => Ok(OutputFormat::Csv),
            "j" | "J" | "json" | "Json" | "JSON" => Ok(OutputFormat::Json),
            "toml" | "Toml" | "TOML" => Ok(OutputFormat::Toml),
            "tsv" | "Tsv" | "TSV" => Ok(OutputFormat::Tsv),
            "y" | "Y" | "yaml" | "Yaml" | "YAML" => Ok(OutputFormat::Yaml),
            _ => Err(OutputFormatError::UnknownFormat(s.to_string())),
        }
    }
}
