//! Methods for calculating statistics from HTS records.

use std::{collections::HashMap, io::Read};

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

    /// Get the mutable HashSet of flow cell IDs
    fn mut_flow_cells(&mut self) -> &mut HashMap<String, u64>;

    /// Get the mutable HashSet of instrument IDs
    fn mut_instruments(&mut self) -> &mut HashMap<String, u64>;

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

    /// Process a Sequence Read Archive record
    ///
    // Sequence Read Archive ID will be ignored since there is no
    // way to figure out what the original flow cell IDs were.
    fn process_sra_split_record(&mut self) {}

    /// Process the flow cell ID from an Illumina read name
    fn process_illumina_flowcell(&mut self, fcid: Option<&[u8]>) {
        if let Some(mut s) = fcid {
            let mut fcid = String::new();

            // wait until the last possible moment to store the flow cell ID as a string
            if s.read_to_string(&mut fcid).is_ok() {
                // track that this flow cell is used
                match self.mut_flow_cells().get_mut(&fcid) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        self.mut_flow_cells().insert(fcid, 1);
                    }
                }
            }
        }
    }

    /// Process the instrument ID from an Illumian read
    fn process_illumina_instrument(&mut self, inst: Option<&[u8]>) {
        if let Some(mut s) = inst {
            let mut instrument_name = String::new();

            // wait until the last possible moment to store the instrument name as a string
            if s.read_to_string(&mut instrument_name).is_ok() {
                // track that the instrument is being used
                match self.mut_instruments().get_mut(&instrument_name) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        self.mut_instruments().insert(instrument_name, 1);
                    }
                }
            }
        }
    }

    /// Process an Illumina (Casava < v1.8) formatted FASTQ record
    fn process_illumina_pre_v1_8_split_record(&mut self) {
        todo!()
    }
}
