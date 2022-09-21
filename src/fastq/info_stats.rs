//! Statistics for a FASTQ file.

use super::FastqInfoOpts;
use crate::record::{
    error::RecordError,
    header::{RecordName, ILLUMINA_SEPARATOR_ASCII_CODE, RNAME_SEPARATOR_ASCII_CODE},
    stats::RecordStats,
};
use needletail::{errors::ParseError, parser::SequenceRecord};
use std::collections::HashMap;

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
    /// Process an Illumina (Casava >= v1.8) formatted FASTQ record
    fn process_illumina_split_record(&mut self, rname: &[u8], opts: &FastqInfoOpts) {
        // Illumina Casava >= v1.8 format
        let mut id_splits = rname.split(|x| *x == ILLUMINA_SEPARATOR_ASCII_CODE);

        // instrument name
        let inst = id_splits.next();
        if opts.instruments {
            self.process_illumina_instrument(inst);
        }

        // run ID
        let run_id = id_splits.next();

        // flow cell ID
        let fcid = id_splits.next();
        if opts.flow_cell_ids {
            self.process_illumina_flowcell(fcid);
        }
    }
}

impl<'a> RecordStats<'a> for FastqStats {
    type Record = SequenceRecord<'a>;
    type Error = ParseError;
    type InfoOpts = FastqInfoOpts;

    /// Create a new set of statistics for a FASTQ file
    fn new() -> Self {
        FastqStats {
            valid_records: 0,
            invalid_records: 0,
            bases: 0,
            lengths: HashMap::new(),
            instruments: HashMap::new(),
            flow_cell_ids: HashMap::new(),
        }
    }

    fn n_valid(&self) -> u64 {
        self.valid_records
    }

    fn n_invalid(&self) -> u64 {
        self.invalid_records
    }

    fn mut_lengths(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.lengths
    }

    fn mut_flow_cells(&mut self) -> &mut HashMap<String, u64> {
        &mut self.flow_cell_ids
    }

    fn mut_instruments(&mut self) -> &mut HashMap<String, u64> {
        &mut self.instruments
    }

    fn process_valid_record(&mut self, seq: &SequenceRecord, opts: &FastqInfoOpts) {
        self.valid_records += 1;

        let seq_length: u64 = seq.num_bases().try_into().unwrap();
        self.bases += seq_length;

        if opts.lengths {
            self.update_lengths(seq_length);
        }
        if opts.flow_cell_ids || opts.instruments {
            if opts.flow_cell_ids || opts.instruments {
                match RecordName::try_from(seq.id()) {
                    Ok(RecordName::CasavaV1_8) => {
                        let mut splits = seq.id().split(|x| *x == RNAME_SEPARATOR_ASCII_CODE);
                        let a = splits.next().unwrap();
                        self.process_illumina_split_record(a, opts);
                    }
                    Ok(RecordName::SequenceReadArchive) => {
                        self.process_sra_split_record();
                    }
                    Err(RecordError::UncertainRecordNameFormat) => todo!(),
                }
            }
        }
    }

    fn process_invalid_record(&mut self) {
        self.invalid_records += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let expected = 4;
        let observed = 2 + 2;

        assert_eq!(expected, observed);
    }
}
