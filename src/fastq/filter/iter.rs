//! Helper structs and methods for iterating through a FASTQ file and IDs.

use super::{FastqFilterError, FastqFilterOpts};
use needletail::parser::{FastqReader, FastxReader, SequenceRecord};
use std::{
    fs::File,
    io::{BufReader, Lines},
};

/// Helper struct for iterating through a FASTQ file and IDs.
pub struct FastqFilterIter<'a> {
    /// Line reader for the IDs to be filtered
    id_file: Lines<BufReader<File>>,

    /// FASTQ record reader
    reader: FastqReader<File>,

    /// The previous ID that was handled
    prev_id: &'a str,

    /// The current ID being handled
    curr_id: &'a str,

    /// The previous record being handled
    prev_record: &'a SequenceRecord<'a>,

    /// The current record being handled
    curr_record: &'a SequenceRecord<'a>,
}

impl<'a> TryFrom<&FastqFilterOpts> for FastqFilterIter<'a> {
    type Error = FastqFilterError;

    fn try_from(value: &FastqFilterOpts) -> Result<Self, Self::Error> {
        let id_file = value.get_id_file_lines()?;

        // first ID in the ID file
        let mut prev_id = match id_file.next() {
            Some(Ok(id)) => id.to_lowercase(),
            Some(Err(_)) => return Err(FastqFilterError::CannotParseFirstIdFileLine),
            None => return Err(FastqFilterError::EmptyIdFile),
        };

        // copy the first ID for setup
        let mut curr_id = prev_id.clone();

        // Reader for the FASTQ file
        let mut reader = value.get_hts_reader();

        // name of the first record in the FASTQ file
        let mut prev_record = match reader.next() {
            Some(Ok(seq)) => seq,
            Some(Err(_)) => return Err(FastqFilterError::CannotParseFirstFastqRecord),
            None => return Err(FastqFilterError::EmptyFastqFile),
        };

        let curr_record = prev_record.clone();

        Ok(Self {
            id_file,
            reader,
            prev_id: prev_id.as_str(),
            curr_id: curr_id.as_str(),
            prev_record: &&prev_record,
            curr_record: &curr_record,
        })
    }
}
