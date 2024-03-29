//! Error handling when filtering records from a FASTQ file.

use std::string::FromUtf8Error;

use needletail::errors::ParseError;
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

    #[error("Cannot parse record in FASTQ file because of the following error. {0}")]
    CannotParseFastqRecord(ParseError),

    #[error("Cannot parse record ID in FASTQ file because of the following error. {0}")]
    CannotParseFastqRecordId(FromUtf8Error),

    #[error("FASTQ file is empty.")]
    EmptyFastqFile,

    #[error("IDs are not sorted. Please sort with `sort -n`.")]
    IdFileNotSorted,

    #[error("FASTQ is not sorted. Please sort with `(z)cat | paste - - - - | sort -n | tr -s \"\t\" \"\n\"`.")]
    FastqNotSorted,

    #[error("Cannot extract a record ID from `None` option.")]
    NoRecordIdFromNone,
}
