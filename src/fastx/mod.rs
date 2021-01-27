use needletail::{parse_fastx_file, FastxReader};
use prettytable::{cell, format, row, Table};
use std::collections::{BTreeMap, HashSet};
use std::string::String;

use crate::utils::HtsFile;

/// Helper function for the most efficient looping over the Fastx file
fn count_info(reader: &mut Box<dyn FastxReader>) -> BTreeMap<String, String> {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases

    while let Some(record) = reader.next() {
        let seq = record.expect("invalid record");
        // add to n_records count
        n_records += 1;
        // keep track of the n_records number of bases
        let seq_length = seq.seq().len() as u32;
        n_bases += seq_length;
    }

    let mut res: BTreeMap<String, String> = BTreeMap::new();
    res.insert("Records".to_string(), n_records.to_string());
    res.insert("Total Bases".to_string(), n_bases.to_string());
    res
}

/// Helper function for the most efficient looping over the Fastx file
fn count_info_lengths(reader: &mut Box<dyn FastxReader>) -> BTreeMap<String, String> {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut record_lens: HashSet<u32> = HashSet::new(); // read lengths

    while let Some(record) = reader.next() {
        let seq = record.expect("invalid record");
        // add to n_records count
        n_records += 1;
        // keep track of the n_records number of bases
        let seq_length = seq.seq().len() as u32;
        n_bases += seq_length;
        if !record_lens.contains(&seq_length) {
            record_lens.insert(seq_length);
        }
    }

    let mut res: BTreeMap<String, String> = BTreeMap::new();
    res.insert("Records".to_string(), n_records.to_string());
    res.insert("Total Bases".to_string(), n_bases.to_string());

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

/// Print information about the FASTX file
pub fn info(hts: &HtsFile, count_lengths: bool) {
    // parse the FASTQ
    let mut reader = parse_fastx_file(hts.path()).expect("Error opening HTS file");

    // decide which helper function to use for most efficient executing
    // can't use a filter_map because the reader doesn't implement the Iterator trait
    let res = match count_lengths {
        false => count_info(&mut reader),
        true => count_info_lengths(&mut reader),
    };

    // construct a table for display
    let mut tab = Table::new();
    tab.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    tab.set_titles(row!["Statistic", "Value"]);

    // iterate over all elements in the different iteration implementations
    for (k, v) in &res {
        tab.add_row(row![k, v]);
    }

    // print to STDOUT
    tab.printstd();
}
