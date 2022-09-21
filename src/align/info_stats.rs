//! Statistics for a SAM/BAM/CRAM file.

use crate::{
    record::header::{ILLUMINA_SEPARATOR_ASCII_CODE, RNAME_SEPARATOR_ASCII_CODE},
    record::{error::RecordError, header::RecordName, stats::RecordStats},
};
use std::{collections::HashMap, io};

use super::SamBamCramInfoOpts;

/// Important statistics from a SAM/BAM/CRAM file.
#[derive(Debug)]
pub struct SamBamCramStats {
    /// Number of valid records.
    valid_records: u64,

    /// Number of invalid records.
    invalid_records: u64,

    /// Total number of bases from these alignments (multi-mapping reads are not double-counted).
    bases: u64,

    /// Length distribution of records
    lengths: HashMap<u64, u64>,

    /// Sequencing instruments
    instruments: HashMap<String, u64>,

    /// Flow cell IDs
    flow_cell_ids: HashMap<String, u64>,

    /// How deep the coverage is from these records.
    genome_depth: (),

    /// What amount of the genome is supported by these records.
    genome_support: (),
}

impl SamBamCramStats {
    /// Process an Illumina (Casava >= v1.8) formatted FASTQ record
    fn process_illumina_split_record(&mut self, rname: &[u8], opts: &SamBamCramInfoOpts) {
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

impl<'a> RecordStats<'a> for SamBamCramStats {
    type Record = bam::Record;
    type Error = io::Error;
    type InfoOpts = SamBamCramInfoOpts;

    /// Create a new set of statistics for a SAM/BAM/CRAM file
    fn new() -> Self {
        SamBamCramStats {
            valid_records: 0,
            invalid_records: 0,
            bases: 0,
            lengths: HashMap::new(),
            instruments: HashMap::new(),
            flow_cell_ids: HashMap::new(),
            genome_depth: (),
            genome_support: (),
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

    /// Process the statistics for a valid record
    fn process_valid_record(&mut self, seq: &Self::Record, opts: &Self::InfoOpts) {
        self.valid_records += 1;

        let seq_length: u64 = seq.query_len().try_into().unwrap();
        self.bases += seq_length;

        if opts.lengths {
            self.update_lengths(seq_length);
        }

        if opts.flow_cell_ids || opts.instruments {
            match RecordName::try_from(seq.name()) {
                Ok(RecordName::CasavaV1_8) => {
                    let mut splits = seq.name().split(|x| *x == RNAME_SEPARATOR_ASCII_CODE);
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

    fn process_invalid_record(&mut self) {
        self.invalid_records += 1;
    }
}
