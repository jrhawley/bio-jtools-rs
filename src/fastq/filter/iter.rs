//! Helper structs and methods for iterating through a FASTQ file and IDs.

use super::FastqFilterError;
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
    pub fn prev_filter_id(&self) -> Option<&[u8]> {
        match self.prev_id {
            Some(id) => Some(id.as_bytes()),
            None => None,
        }
    }

    /// Retrieve the current ID in the filter file
    pub fn curr_filter_id(&self) -> Option<&[u8]> {
        match self.curr_id {
            Some(id) => Some(id.as_bytes()),
            None => None,
        }
    }

    /// Retrieve the ID of the previous record
    pub fn prev_record_id(&self) -> Option<&[u8]> {
        match &self.prev_record {
            Some(r) => Some(r.id()),
            None => None,
        }
    }

    /// Retrieve the ID of the current record
    pub fn curr_record_id(&self) -> Option<&[u8]> {
        match &self.curr_record {
            Some(r) => Some(r.id()),
            None => None,
        }
    }

    /// Retrieve the current record being processed
    pub fn curr_record(&self) -> Option<&SequenceRecord> {
        self.curr_record.as_ref()
    }

    /// Retrieve the next ID form the ID file
    pub fn get_next_id(
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
    pub fn get_next_record(
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

    /// Check that the the file IDs are in order
    pub fn assert_ids_are_sorted(&self) -> Result<(), FastqFilterError> {
        match (self.curr_filter_id(), self.prev_filter_id()) {
            (Some(curr), Some(prev)) => {
                if curr < prev {
                    Ok(())
                } else {
                    Err(FastqFilterError::IdFileNotSorted)
                }
            }
            (_, _) => Ok(()),
        }
    }

    /// Check that the the file IDs are in order
    pub fn assert_records_are_sorted(&self) -> Result<(), FastqFilterError> {
        match (self.curr_record_id(), self.prev_record_id()) {
            (Some(curr), Some(prev)) => {
                if curr < prev {
                    Ok(())
                } else {
                    Err(FastqFilterError::FastqNotSorted)
                }
            }
            (_, _) => Ok(()),
        }
    }
}
