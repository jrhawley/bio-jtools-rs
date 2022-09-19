//! Statistics for a SAM/BAM/CRAM file.

use bam::RecordReader;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io,
};

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
