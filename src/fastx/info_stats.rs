//! Statistics to record when parsing info about an HTS file

use needletail::{errors::ParseError, parser::SequenceRecord};
use std::{collections::HashMap, io::Read};

use super::InfoOpts;

/// Statistics from a FASTQ file
#[derive(Debug)]
pub(crate) struct FastqStats {
    /// Total number of valid records
    valid_records: u64,

    /// Total number of invalid records
    invalid_records: u64,

    /// Total number of bases in a file
    bases: u64,

    /// Length distribution of records
    lengths: HashMap<u64, u64>,

    /// Sequencing instruments
    instruments: HashMap<String, u64>,

    /// Flow cell IDs
    flow_cell_ids: HashMap<String, u64>,
}

impl FastqStats {
    /// Create a new set of statistics for a FASTQ file
    pub(crate) fn new() -> Self {
        FastqStats {
            valid_records: 0,
            invalid_records: 0,
            bases: 0,
            lengths: HashMap::new(),
            instruments: HashMap::new(),
            flow_cell_ids: HashMap::new(),
        }
    }

    /// Get the total number of valid records
    pub(crate) fn n_valid(&self) -> u64 {
        self.valid_records
    }

    /// Get the total number of invalid records
    pub(crate) fn n_invalid(&self) -> u64 {
        self.invalid_records
    }

    /// Get the total number of records processed
    pub(crate) fn n_records(&self) -> u64 {
        self.n_valid() + self.n_invalid()
    }

    /// Process a single record from a FASTQ file to record its statistics
    pub(crate) fn process_record(
        &mut self,
        rec: &Result<SequenceRecord, ParseError>,
        opts: &InfoOpts,
    ) {
        match rec {
            Ok(seq) => {
                self.valid_records += 1;

                let seq_length: u64 = seq.num_bases().try_into().unwrap();
                self.bases += seq_length;

                if opts.lengths {
                    match self.lengths.get_mut(&seq_length) {
                        Some(v) => {
                            *v += 1;
                        }
                        None => {
                            self.lengths.insert(seq_length, 1);
                        }
                    }
                }

                if opts.flow_cell_ids || opts.instruments {
                    // split the byte string by " "
                    let mut splits = seq.id().split(|x| *x == 32);

                    match (splits.next(), splits.next(), splits.next()) {
                        (Some(a), Some(b), Some(c)) => {
                            // Sequence Read Archive ID will be ignored since there is no
                            // way to figure out what the original flow cell IDs were
                        }
                        (Some(a), Some(b), None) => {
                            // Illumina Casava >= v1.8 format
                            // split the first element of the byte string by ":"
                            let mut id_splits = a.split(|x| *x == 58);

                            // instrument name
                            if opts.instruments {
                                if let Some(mut s) = id_splits.next() {
                                    let mut instrument_name = String::new();

                                    // wait until the last possible moment to store the instrument name as a string
                                    s.read_to_string(&mut instrument_name);

                                    // track that the instrument is being used
                                    match self.instruments.get_mut(&instrument_name) {
                                        Some(v) => {
                                            *v += 1;
                                        }
                                        None => {
                                            self.instruments.insert(instrument_name, 1);
                                        }
                                    }
                                }
                            }

                            // run ID
                            id_splits.next();

                            // flow cell ID
                            if opts.flow_cell_ids {
                                if let Some(mut s) = id_splits.next() {
                                    let mut fcid = String::new();

                                    // wait until the last possible moment to store the flow cell ID as a string
                                    s.read_to_string(&mut fcid);

                                    // track that this flow cell is used
                                    match self.flow_cell_ids.get_mut(&fcid) {
                                        Some(v) => {
                                            *v += 1;
                                        }
                                        None => {
                                            self.flow_cell_ids.insert(fcid, 1);
                                        }
                                    }
                                }
                            }
                        }
                        (Some(a), None, None) => todo!(),
                        _ => todo!(),
                    };
                }
            }
            Err(_) => {
                self.invalid_records += 1;
            }
        }
    }
}
