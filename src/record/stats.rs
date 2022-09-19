//! Methods for calculating statistics from HTS records.

use std::collections::HashMap;

pub trait RecordStats<'a> {
    type Record;
    type Error;
    type InfoOpts;

    /// Create a new collection of statistics
    fn new() -> Self;

    /// Get the total number of valid records
    fn n_valid(&self) -> u64;

    /// Get the total number of invalid records
    fn n_invalid(&self) -> u64;

    /// Get the total number of records processed
    fn n_records(&self) -> u64 {
        self.n_valid() + self.n_invalid()
    }

    /// Get the mutable HashMap of lengths
    fn mut_lengths(&mut self) -> &mut HashMap<u64, u64>;

    /// Update the information about the lengths of records
    fn update_lengths(&mut self, seq_length: u64) {
        if let Some(v) = self.mut_lengths().get_mut(&seq_length) {
            *v += 1;
        } else {
            self.mut_lengths().insert(seq_length, 1);
        }
    }

    /// Process a single record from an HTS file to record its statistics
    fn process_record(&mut self, rec: &Result<Self::Record, Self::Error>, opts: &Self::InfoOpts) {
        if let Ok(seq) = rec {
            self.process_valid_record(seq, opts);
        } else {
            self.process_invalid_record();
        }
    }

    /// Process the statistics for a valid record
    fn process_valid_record(&mut self, seq: &Self::Record, opts: &Self::InfoOpts);

    /// Process the statistics for an invalid record
    fn process_invalid_record(&mut self);
}
