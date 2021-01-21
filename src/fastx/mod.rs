use needletail::parse_fastx_file;
use prettytable::{cell, format, row, Table};
use std::collections::HashSet;
use std::str;
use std::string::String;

use crate::utils::HtsFile;

/// Print information about the FASTX file
pub fn info(hts: &HtsFile) {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut instruments: HashSet<String> = HashSet::new(); // sequencing instrument IDs
    let mut record_lens: HashSet<u32> = HashSet::new(); // read lengths

    // parse the FASTQ
    let mut reader = parse_fastx_file(hts.path()).expect("Error opening HTS file");
    while let Some(record) = reader.next() {
        let seq = record.expect("invalid record");
        // add to n_records count
        n_records += 1;
        // keep track of the n_records number of bases
        let seq_length = seq.seq().len() as u32;
        n_bases += seq_length;
        // parse the instruments machine ID (always at the beginning of a read ID for the new Illumina encodings)
        let id_val = str::from_utf8(
            seq.id()
            .split(|c| c == &b':' || c == &b' ' || c == &b'.')
            .next()
            .unwrap(),
        )
        .unwrap();
        // if a new intrument ID is detected
        if !instruments.contains(id_val) {
            instruments.insert(id_val.to_string());
        }
        if !record_lens.contains(&seq_length) {
            record_lens.insert(seq_length);
        }
    }

    // format a string of all the instruments found
    let instruments = instruments
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>(); // first convert to Vec for easy slicing
    let mut inst_str = String::from(instruments[0]);
    for inst in &instruments[1..] {
        inst_str.push_str(format!(", {}", inst).as_str());
    }
    // format a string of all the record lengths
    let lengths = record_lens
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>(); // first convert to Vec for easy slicing
    let mut len_str = String::from(&lengths[0]);
    for l in &lengths[1..] {
        len_str.push_str(format!(", {}", &l).as_str());
    }

    // construct a table for display
    let mut tab = Table::new();
    tab.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    tab.set_titles(row!["Statistic", "Value"]);

    tab.add_row(row!["Records", n_records]);
    tab.add_row(row!["Record Lengths", len_str]);
    tab.add_row(row!["Total Bases", n_bases]);
    tab.add_row(row!["Instruments", inst_str]);

    // print to STDOUT
    tab.printstd();
}
