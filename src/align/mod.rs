//! # Processing SAM, BAM, and CRAM alignment files
//! Functions and methods related to processing alignment files, such as [SAM, BAM](https://samtools.github.io/hts-specs/SAMv1.pdf), and [CRAM](https://samtools.github.io/hts-specs/CRAMv3.pdf) files.

use bam::{Record, RecordReader, RecordWriter};
use prettytable::{cell, format, row, Table};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// Helper function for the most efficient looping over the Fastx file
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

    // construct a table for display
    let mut tab = Table::new();
    tab.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    tab.set_titles(row!["Statistic", "Value"]);
    for (k, v) in &res {
        tab.add_row(row![k, v]);
    }

    // print to STDOUT
    tab.printstd();
}

/// Filter out reads according to a list of IDs
/// Assumes a sorted SAM/BAM file and a sorted list of IDs
/// # Arguments
/// * reader: RecordReader for a name-sorted SAM/BAM file. Sort with `samtools sort -n`
/// * ids: A name-sorted file containing IDs to filter out (or keep) from the SAM/BAM file. Sort with `sort ids.in > ids.filtered.out`.
/// * out: Output file to write filtered reads to
/// * keep: Boolean to keep the reads matching IDs in `ids` (`true`) or discard them (`false`)
pub fn filter<T: RecordReader, S: RecordWriter>(
    reader: &mut T,
    ids: &Path,
    writer: &mut S,
    keep: bool,
) {
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

    // name of the first record in the SAM/BAM file
    let mut prev_record = Record::new();
    match reader.read_into(&mut prev_record) {
        // no problem if reading the first read
        Ok(true) => {}
        Ok(false) => panic!("No reads in HTS file"),
        Err(_) => panic!("Error parsing first read in HTS file"),
    };
    let mut prev_record_name = from_utf8(&prev_record.name()).unwrap().to_lowercase();
    let mut cur_record = prev_record.clone();
    let mut cur_record_name = prev_record_name.clone();

    let mut deal_with_remaining_reads = false;

    // step through recordsand IDs
    loop {
        // panic if IDs aren't sorted
        if &cur_id < &prev_id {
            panic!("IDs aren't sorted. Please sort with `sort ids.in > ids.filtered.out`")
        }
        // panic if SAM/BAM isn't name-sorted
        if &cur_record_name < &prev_record_name {
            panic!("HTS file isn't sorted. Please sort with `samtools sort -n`")
        }

        // decide what to do with cur_record, depending on how it relates to cur_id
        // write or discard record if the IDs are ahead of the reads
        if &cur_record_name < &cur_id {
            if !keep {
                writer.write(&cur_record).unwrap();
            }
            // update the records
            prev_record_name = cur_record_name;
            // check if there is a subsequent record in the SAM/BAM
            match reader.read_into(&mut cur_record) {
                Ok(true) => {}
                // if no more records in SAM/BAM, close the writer and exit the loop
                Ok(false) => {
                    writer.finish().unwrap();
                    break;
                }
                Err(_) => panic!("Error parsing record in HTS file"),
            }
            cur_record_name = from_utf8(&cur_record.name()).unwrap().to_lowercase();
        // update the IDs to catch up to the records
        } else if cur_record_name > cur_id {
            match id_file.next() {
                // update the IDs
                Some(Ok(id)) => {
                    prev_id = cur_id;
                    cur_id = id.to_lowercase();
                }
                Some(Err(_)) => panic!("Error parsing ID in ID file {}.", ids.display()),
                // if no more IDs, close this reader and deal with the remaining reads outside the loop
                None => {
                    // write the current read, if required, then deal with all the future ones
                    writer.write(&cur_record).unwrap();
                    deal_with_remaining_reads = true;
                    break;
                }
            };
        } else {
            // don't purge this ID yet, just move onto the next record
            // there may be other records that match this ID (e.g. mate or non-unique alignment)
            if keep {
                writer.write(&cur_record).unwrap();
            }
            prev_record_name = cur_record_name;
            match reader.read_into(&mut cur_record) {
                // if there is a subsequent records in the SAM/BAM
                Ok(true) => {
                    cur_record_name = from_utf8(&cur_record.name()).unwrap().to_lowercase();
                }
                // if no more reads in SAM/BAM, close the writer and exit the loop
                Ok(false) => {
                    writer.finish().unwrap();
                    break;
                }
                Err(_) => panic!("Error parsing record in HTS file"),
            }
        }
    }

    // if all of the IDs have been exhausted but we still have records to write
    // write them without comparing against IDs
    if deal_with_remaining_reads && !keep {
        for read in reader {
            let record = read.unwrap();
            writer.write(&record).unwrap();
        }
        writer.finish().unwrap();
    }
}
