//! Methods for calculating statistics from HTS records.

pub trait RecordStats {
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

    /// Process the statistics for an invalid record
    fn process_invalid_record(&mut self);
}
