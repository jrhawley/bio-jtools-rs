//! Error handling when filtering records from a FASTQ file.

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FastqFilterError {
    #[error("Cannot specify both a regular expression and a file with exact IDs.")]
    CannotSpecifyRegexAndIdFile,

    #[error("You must filter against something, like a regular expression or file containing exact IDs.")]
    FilterCannotBeEmpty,

    #[error("ID file is required but not provided.")]
    IdFileNotProvidedWhenRequired,

    #[error("ID file cannot be opened.")]
    IdFileCannotBeOpened,

    #[error("Cannot parse first line in ID file.")]
    CannotParseIdFileLine,

    #[error("ID file is empty.")]
    EmptyIdFile,

    #[error("Cannot parse first record in FASTQ file.")]
    CannotParseFastqRecord,

    #[error("FASTQ file is empty.")]
    EmptyFastqFile,
}
