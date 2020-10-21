use bam::{RecordReader, RecordWriter};
use prettytable::{cell, format, row, Table};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// Print information about the alignment file
pub fn info<T: RecordReader>(reader: &mut T) {
    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut n_errs: u32 = 0; // number of alignments resulting in parse errors

    // parse the alignment file
    for record in reader {
        match record {
            Ok(rec) => {
                // add to n_records count
                n_records += 1;
                // keep track of the n_records number of bases
                n_bases += rec.sequence().len() as u32;
            }
            Err(_) => n_errs += 1,
        }
    }

    // construct a table for display
    let mut tab = Table::new();
    tab.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    tab.set_titles(row!["Statistic", "Value"]);

    tab.add_row(row!["Records", n_records]);
    tab.add_row(row!["Bases", n_bases]);
    tab.add_row(row!["Errors", n_errs]);

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
    let reader_iter = reader.into_iter();
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
    // name of the first read in the SAM/BAM file
    let mut prev_read = match reader_iter.next() {
        Some(Ok(read)) => read,
        Some(Err(_)) => panic!("Error parsing first read in HTS file"),
        None => panic!("No reads in HTS file"),
    };
    let mut prev_read_name = from_utf8(prev_read.name()).unwrap().to_lowercase();
    let mut cur_read = prev_read;
    let mut cur_read_name = from_utf8(cur_read.name()).unwrap().to_lowercase();

    let mut deal_with_remaining_reads = false;

    // step through reads and IDs
    loop {
        // panic if IDs aren't sorted
        if &cur_id < &prev_id {
            panic!("IDs aren't sorted. Please sort with `sort ids.in > ids.filtered.out`")
        }
        // panic if SAM/BAM isn't name-sorted
        if &cur_read_name < &prev_read_name {
            panic!("HTS file isn't sorted. Please sort with `samtools sort -n`")
        }

        // decide what to do with cur_read, depending on how it relates to cur_id
        // write or discard read if the IDs are ahead of the reads
        if &cur_read_name < &cur_id {
            // look for next read
            match reader_iter.next() {
                // update the reads if there is a subsequent read in the SAM/BAM
                Some(Ok(read)) => {
                    if !keep {
                        writer.write(&cur_read).unwrap();
                    }
                    prev_read = cur_read;
                    prev_read_name = cur_read_name;
                    cur_read = read;
                    cur_read_name = from_utf8(cur_read.name()).unwrap().to_lowercase();
                }
                Some(Err(_)) => panic!("Error parsing record in HTS file"),
                // if no more reads in SAM/BAM, close the writer and exit the loop
                None => {
                    writer.finish().unwrap();
                    break;
                }
            };
        // update the IDs to catch up to the reads
        } else if cur_read_name > cur_id {
            match id_file.next() {
                // update the IDs
                Some(Ok(id)) => {
                    prev_id = cur_id;
                    cur_id = id.to_lowercase();
                }
                Some(Err(_)) => panic!("Error parsing ID in ID file {}.", ids.display()),
                // if no more IDs, close this reader and deal with the remaining reads outside the loop
                None => {
                    deal_with_remaining_reads = true;
                    break;
                }
            };
        } else {
            // don't purge this ID yet, just move onto the next read
            // there may be other alignments that match this ID (e.g. mate or non-unique alignment)
            match reader_iter.next() {
                // update the reads if there is a subsequent read in the SAM/BAM
                Some(Ok(read)) => {
                    if keep {
                        writer.write(&cur_read).unwrap();
                    }
                    prev_read = cur_read;
                    prev_read_name = cur_read_name;
                    cur_read = read;
                    cur_read_name = from_utf8(cur_read.name()).unwrap().to_lowercase();
                }
                Some(Err(_)) => panic!("Error parsing record in HTS file"),
                // if no more reads in SAM/BAM, close the writer and exit the loop
                None => {
                    writer.finish().unwrap();
                    break;
                }
            }
        }
    }

    // if all of the IDs have been exhausted but we still have reads to write
    // write them without comparing against IDs
    if deal_with_remaining_reads && !keep {
        for read in reader_iter {
            let record = read.unwrap();
            writer.write(&record).unwrap();
        }
        writer.finish().unwrap();
    }
}
