//! Filter out reads from a FASTQ file.

pub mod error;
pub mod iter;
pub mod old_filter;
pub mod opts;

pub use error::FastqFilterError;
pub use opts::FastqFilterOpts;
