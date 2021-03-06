//! # Process and organize sequencing batches and other bulk data
//! Functions and methods related to processing raw sequencing files, such as [FASTA](https://en.wikipedia.org/wiki/FASTA_format) and [FASTQ](https://en.wikipedia.org/wiki/FASTQ_format) files.

use core::panic;
use needletail::{parse_fastx_file, FastxReader};
use prettytable::{cell, format, row, Table};
use std::io::{BufRead, BufReader, BufWriter};
use std::{
    collections::{BTreeMap, HashSet},
    str::from_utf8,
};
use std::{fs::File, path::Path, string::String};

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

/// Filter out reads according to a list of IDs
/// Assumes a sorted Fastx file and a sorted list of IDs
/// # Arguments
/// * hts: HtsFile for a name-sorted Fastx file. Sort with `(z)cat | paste | sort -n`
/// * ids: A name-sorted file containing IDs to filter out (or keep) from the Fastx file. Sort with `sort ids.in > ids.filtered.out`.
/// * out: Output file to write filtered reads to
/// * keep: Boolean to keep the reads matching IDs in `ids` (`true`) or discard them (`false`)
pub fn filter(hts: &HtsFile, ids: &Path, out_hts: &Path, keep: bool) {
    // open IDs to filter
    let mut id_file = match File::open(ids) {
        Ok(f) => BufReader::new(f).lines(),
        Err(_) => panic!("IDs file {} could not be opened.", ids.display()),
    };

    // first ID in the ID file
    let mut prev_id = match id_file.next() {
        Some(Ok(id)) => id.to_lowercase(),
        Some(Err(_)) => panic!("Error parsing first line in ID file {}.", ids.display()),
        None => panic!("No IDs in ID file {}. No need to filter", ids.display()),
    };
    let mut cur_id = prev_id.clone();

    // parse the FASTQ
    let mut reader = parse_fastx_file(hts.path()).expect("Error opening HTS file");

    // name of the first record in the Fastx file
    let mut prev_record = match reader.next() {
        Some(Ok(seq)) => seq,
        Some(Err(_)) => panic!("Error parsing first record in HTS file"),
        None => panic!("No records in HTS file"),
    };
    let mut prev_record_name = from_utf8(&prev_record.id()).unwrap().to_lowercase();
    let mut cur_record = prev_record.clone();
    let mut cur_record_name = prev_record_name.clone();

    println!("{}", &cur_id);
    println!("{}", &cur_record_name);

    let mut deal_with_remaining_reads = false;

    // writer for the output Fastx file
    let writer = match File::create(out_hts) {
        Ok(f) => BufWriter::new(f),
        Err(e) => panic!("{}", e),
    };

    //     // step through records and IDs
    //     while let Some(record) = reader.next() {
    //         let seq = record.expect("invalid record");
    //     }

    //     loop {
    //         // panic if IDs aren't sorted
    //         if &cur_id < &prev_id {
    //             panic!("IDs aren't sorted. Please sort with `(z)cat | paste | sort -n`")
    //         }
    //         // panic if SAM/BAM isn't name-sorted
    //         if &cur_record_name < &prev_record_name {
    //             panic!("HTS file isn't sorted. Please sort with `(z)cat {input} | paste - - - - | sort | tr -s "\t" "\n" > {input}.sorted.fastq`")
    //         }

    //         // decide what to do with cur_record, depending on how it relates to cur_id
    //         // write or discard record if the IDs are ahead of the reads
    //         if &cur_record_name < &cur_id {
    //             if !keep {
    //                 writer.write(&cur_record).unwrap();
    //             }
    //             // update the records
    //             prev_record_name = cur_record_name;
    //             // check if there is a subsequent record in the SAM/BAM
    //             match reader.read_into(&mut cur_record) {
    //                 Ok(true) => {}
    //                 // if no more records in SAM/BAM, close the writer and exit the loop
    //                 Ok(false) => {
    //                     writer.finish().unwrap();
    //                     break;
    //                 }
    //                 Err(_) => panic!("Error parsing record in HTS file"),
    //             }
    //             cur_record_name = from_utf8(&cur_record.name()).unwrap().to_lowercase();
    //         // update the IDs to catch up to the records
    //         } else if cur_record_name > cur_id {
    //             match id_file.next() {
    //                 // update the IDs
    //                 Some(Ok(id)) => {
    //                     prev_id = cur_id;
    //                     cur_id = id.to_lowercase();
    //                 }
    //                 Some(Err(_)) => panic!("Error parsing ID in ID file {}.", ids.display()),
    //                 // if no more IDs, close this reader and deal with the remaining reads outside the loop
    //                 None => {
    //                     // write the current read, if required, then deal with all the future ones
    //                     writer.write(&cur_record).unwrap();
    //                     deal_with_remaining_reads = true;
    //                     break;
    //                 }
    //             };
    //         } else {
    //             // don't purge this ID yet, just move onto the next record
    //             // there may be other records that match this ID (e.g. mate or non-unique alignment)
    //             if keep {
    //                 writer.write(&cur_record).unwrap();
    //             }
    //             prev_record_name = cur_record_name;
    //             match reader.read_into(&mut cur_record) {
    //                 // if there is a subsequent records in the SAM/BAM
    //                 Ok(true) => {
    //                     cur_record_name = from_utf8(&cur_record.name()).unwrap().to_lowercase();
    //                 }
    //                 // if no more reads in SAM/BAM, close the writer and exit the loop
    //                 Ok(false) => {
    //                     writer.finish().unwrap();
    //                     break;
    //                 }
    //                 Err(_) => panic!("Error parsing record in HTS file"),
    //             }
    //         }
    //     }

    //     // if all of the IDs have been exhausted but we still have records to write
    //     // write them without comparing against IDs
    //     if deal_with_remaining_reads && !keep {
    //         for read in reader {
    //             let record = read.unwrap();
    //             writer.write(&record).unwrap();
    //         }
    //         writer.finish().unwrap();
    //     }
}
