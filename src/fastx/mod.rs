// use fastq::parse_path;
use needletail::parse_sequence_path;
use std::collections::HashSet;
use std::path::Path;
use std::str;
use std::string::String;

pub fn info<'a>(path: &Path) {
    // values to keep and display
    let mut n_records: usize = 0; // number of records
    let mut n_bases: usize = 0; // number of bases
    let mut instruments: HashSet<String> = HashSet::new(); // sequencing instrument IDs

    // parse the FASTQ
    parse_sequence_path(
        path,
        |_| {},
        |seq| {
            // add to n_records count
            n_records += 1;
            // keep track of the n_records number of bases
            n_bases += seq.seq.len();
            // parse the instruments machine ID (always at the beginning of a read ID for the new Illumina encodings)
            let id_val = str::from_utf8(
                seq.id
                    .split(|c| c == &b':' || c == &b' ' || c == &b'.')
                    .next()
                    .unwrap(),
            )
            .unwrap();
            // if a new intrument ID is detected
            if !instruments.contains(id_val) {
                instruments.insert(id_val.to_string());
            }
        },
    )
    .expect("Error parsing FASTQ. Possible invalid compression.");

    // print output
    println!("{} bases", n_bases);
    println!("{} reads", n_records);
    println!("Instruments: {:?}", instruments);
}
