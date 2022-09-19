//! Statistics for a SAM/BAM/CRAM file.

use bam::RecordReader;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io,
};

use crate::fastq::header::{ILLUMINA_SEPARATOR_ASCII_CODE, RNAME_SEPARATOR_ASCII_CODE};

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
    /// Create a new set of statistics for a FASTQ file
    pub(crate) fn new() -> Self {
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
        rec: &Result<bam::Record, io::Error>,
        opts: &SamBamCramInfoOpts,
    ) {
        if let Ok(seq) = rec {
            self.process_valid_record(seq, opts);
        } else {
            self.process_invalid_record();
        }
    }

    /// Process the statistics for a valid record
    fn process_valid_record(&mut self, seq: &bam::Record, opts: &SamBamCramInfoOpts) {
        self.valid_records += 1;

        let seq_length: u64 = seq.query_len().try_into().unwrap();
        self.bases += seq_length;

        if opts.lengths {
            self.update_lengths(seq_length);
        }

        if opts.flow_cell_ids || opts.instruments {
            let mut splits = seq.name().split(|x| *x == RNAME_SEPARATOR_ASCII_CODE);

            match (splits.next(), splits.next(), splits.next()) {
                (Some(_), Some(_), Some(_)) => self.process_sra_split_record(),
                (Some(a), Some(_), None) => self.process_illumina_split_record(a, opts),
                (Some(_), None, None) => self.process_illumina_pre_v1_8_split_record(),
                _ => todo!(),
            };
        }
    }

    /// Update the information about the lengths of records
    fn update_lengths(&mut self, seq_length: u64) {
        if let Some(v) = self.lengths.get_mut(&seq_length) {
            *v += 1;
        } else {
            self.lengths.insert(seq_length, 1);
        }
    }

    /// Process the statistics for an invalid record
    fn process_invalid_record(&mut self) {
        self.invalid_records += 1;
    }

    /// Process a Sequence Read Archive FASTQ record
    fn process_sra_split_record(&mut self) {
        // Sequence Read Archive ID will be ignored since there is no
        // way to figure out what the original flow cell IDs were
    }

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

    fn process_illumina_flowcell(&mut self, fcid: Option<&[u8]>) {
        todo!()

    fn process_illumina_instrument(&mut self, inst: Option<&[u8]>) {
        todo!()
    }

    /// Process an Illumina (Casava < v1.8) formatted FASTQ record
    fn process_illumina_pre_v1_8_split_record(&mut self) {
        todo!()
    }
}

/// Helper function for the most efficient looping over a SAM/BAM file
fn count_info<T: RecordReader>(reader: &mut T) -> BTreeMap<String, String> {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut n_errs: u32 = 0; // number of alignments resulting in parse errors

    for record in reader {
        match record {
            Ok(rec) => {
                // add to n_records count
                n_records += 1;
                // keep track of the n_records number of bases
                let seq_length = rec.sequence().len() as u32;
                n_bases += seq_length;
            }
            Err(_) => n_errs += 1,
        }
    }

    let mut res: BTreeMap<String, String> = BTreeMap::new();
    res.insert("Records".to_string(), n_records.to_string());
    res.insert("Total Bases".to_string(), n_bases.to_string());
    res.insert("Errors".to_string(), n_errs.to_string());
    res
}

/// Helper function for the most efficient looping over the Fastx file
fn count_info_lengths<T: RecordReader>(reader: &mut T) -> BTreeMap<String, String> {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut n_errs: u32 = 0; // number of alignments resulting in parse errors
    let mut record_lens: HashSet<u32> = HashSet::new(); // read lengths

    for record in reader {
        match record {
            Ok(rec) => {
                // add to n_records count
                n_records += 1;
                // keep track of the n_records number of bases
                let seq_length = rec.sequence().len() as u32;
                n_bases += seq_length;
                // add to set of seq lengths
                if !record_lens.contains(&seq_length) {
                    record_lens.insert(seq_length);
                }
            }
            Err(_) => n_errs += 1,
        }
    }

    let mut res: BTreeMap<String, String> = BTreeMap::new();
    res.insert("Records".to_string(), n_records.to_string());
    res.insert("Total Bases".to_string(), n_bases.to_string());
    res.insert("Errors".to_string(), n_errs.to_string());

    // format a string of all the record lengths
    let lengths = record_lens
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>(); // first convert to Vec for easy slicing
    let mut len_str = String::from(&lengths[0]);
    for l in &lengths[1..] {
        len_str.push_str(format!(", {}", &l).as_str());
    }
    res.insert("Record Lengths".to_string(), len_str);
    res
}

/// Print information about the alignment file
pub fn info<T: RecordReader>(reader: &mut T, count_lengths: bool) {
    // parse the alignment file
    let res = match count_lengths {
        false => count_info(reader),
        true => count_info_lengths(reader),
    };
}
