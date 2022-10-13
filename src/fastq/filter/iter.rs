//! Helper structs and methods for iterating through a FASTQ file and IDs.

use super::FastqFilterError;
use needletail::parser::{FastqReader, FastxReader, SequenceRecord};
use std::{
    fs::File,
    io::{BufReader, Lines},
};

/// Helper struct for iterating through a FASTQ file and IDs.
pub struct FastqFilterIter<'record> {
    /// The previous ID that was handled
    prev_id: Option<String>,

    /// The current ID being handled
    curr_id: Option<String>,

    /// The ID of the previous record that was recently dealt with
    prev_record: Option<String>,

    /// The current record being handled
    curr_record: Option<SequenceRecord<'record>>,
}

impl<'record> Default for FastqFilterIter<'record> {
    fn default() -> Self {
        Self {
            prev_id: None,
            curr_id: None,
            prev_record: None,
            curr_record: None,
        }
    }
}

impl<'record> FastqFilterIter<'record> {
    /// Create a new iterator object
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve the previous ID in the filter file
    pub fn prev_filter_id(&self) -> Option<&[u8]> {
        match &self.prev_id {
            Some(id) => Some(id.as_bytes()),
            None => None,
        }
    }

    /// Retrieve the current ID in the filter file
    pub fn curr_filter_id(&self) -> Option<&[u8]> {
        match &self.curr_id {
            Some(id) => Some(id.as_bytes()),
            None => None,
        }
    }

    /// Retrieve the ID of the previous record
    pub fn prev_record_id(&self) -> Option<&[u8]> {
        match &self.prev_record {
            Some(r) => Some(r.as_bytes()),
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
        self.prev_id.clone_from(&self.curr_id);
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
        fq_reader: &'record mut FastqReader<File>,
    ) -> Result<(), FastqFilterError> {
        // Instead of storing the entire record, we are going to only store the ID from the record that was just processed.
        // To do this, we create a string of the ID from the byte slice that make up the record's ID.
        // (FASTQ files are ASCII encoded, so from_utf8 should always work).
        // The ID byte slice is copied to a new `Vec`, but this is less expensive than copying the entire contents of the FASTQ record.
        // This is coerced into a String, which is owned by `self` and not `fq_reader`.
        // This avoids having multiple simultaneous mutable references to `fq_reader`.
        self.prev_record = match &self.curr_record {
            Some(rec) => match String::from_utf8(rec.id().to_vec()) {
                Ok(val) => Some(val),
                Err(e) => return Err(FastqFilterError::CannotParseFastqRecordId(e)),
            },
            None => None,
        };

        // advance the next record in the FASTQ file
        self.curr_record = match fq_reader.next() {
            Some(Ok(seq)) => Some(seq),
            Some(Err(e)) => return Err(FastqFilterError::CannotParseFastqRecord(e)),
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
