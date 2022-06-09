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

    /// Flow cell IDs
    flow_cell_ids: HashMap<String, u32>,
}

impl FastqStats {
    /// Create a new set of statistics for a FASTQ file
    pub(crate) fn new() -> Self {
        FastqStats {
            valid_records: 0,
            invalid_records: 0,
            bases: 0,
            lengths: HashMap::new(),
            flow_cell_ids: HashMap::new(),
        }
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

                if opts.flow_cell_ids {
                    let mut id_buf = String::new();
                    seq.id().read_to_string(&mut id_buf);
                    println!("{:#?}", seq.id());
                    println!("{:#?}", id_buf);

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
                            id_splits.next();

                            // run ID
                            id_splits.next();

                            // flow cell ID
                            if let Some(mut s) = id_splits.next() {
                                let mut fcid = String::new();
                                s.read_to_string(&mut fcid);

                                println!("{}", fcid);
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
