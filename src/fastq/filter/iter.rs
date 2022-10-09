//! Helper structs and methods for iterating through a FASTQ file and IDs.

use super::{FastqFilterError, FastqFilterOpts};
use needletail::parser::{FastqReader, FastxReader, SequenceRecord};
use std::{
    fs::File,
    io::{BufReader, Lines},
};

/// Helper struct for iterating through a FASTQ file and IDs.
pub struct FastqFilterIter<'a> {
    /// The previous ID that was handled
    prev_id: Option<String>,

    /// The current ID being handled
    curr_id: Option<String>,

    /// The previous record being handled
    prev_record: Option<SequenceRecord<'a>>,

    /// The current record being handled
    curr_record: Option<SequenceRecord<'a>>,
}

impl<'a> Default for FastqFilterIter<'a> {
    fn default() -> Self {
        Self {
            prev_id: None,
            curr_id: None,
            prev_record: None,
            curr_record: None,
        }
    }
}

impl<'a> FastqFilterIter<'a> {
    /// Create a new iterator object
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve the previous ID in the filter file
    fn prev_filter_id(&self) -> Option<&String> {
        self.prev_id.as_ref()
    }

    /// Retrieve the current ID in the filter file
    fn curr_filter_id(&self) -> Option<&String> {
        self.curr_id.as_ref()
    }

    /// Retrieve the ID of the previous record
    fn prev_record_id(&self) -> Option<&[u8]> {
        match &self.prev_record {
            Some(r) => Some(r.id()),
            None => None,
        }
    }

    /// Retrieve the ID of the current record
    fn curr_record_id(&self) -> Option<&[u8]> {
        match &self.curr_record {
            Some(r) => Some(r.id()),
            None => None,
        }
    }

    /// Retrieve the next ID form the ID file
    fn get_next_id(
        &mut self,
        id_reader: &mut Lines<BufReader<File>>,
    ) -> Result<(), FastqFilterError> {
        self.prev_id = self.curr_id.to_owned();
        self.curr_id = match id_reader.next() {
            Some(Ok(s)) => Some(s),
            Some(Err(_)) => return Err(FastqFilterError::CannotParseIdFileLine),
            None => None,
        };

        Ok(())
    }

    /// Retrieve the next record form the FASTQ file
    fn get_next_record(
        &mut self,
        fq_reader: &'a mut FastqReader<File>,
    ) -> Result<(), FastqFilterError> {
        self.prev_record = self.curr_record.to_owned();
        self.curr_record = match fq_reader.next() {
            Some(Ok(seq)) => Some(seq),
            Some(Err(_)) => return Err(FastqFilterError::CannotParseFastqRecord),
            None => None,
        };

        Ok(())
    }
}
