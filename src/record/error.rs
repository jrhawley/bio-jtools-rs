//! Errors when parsing HTS records.

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RecordError {
    #[error("Could not determine what type of information is encoded in the record name.")]
    UncertainRecordNameFormat,
}
