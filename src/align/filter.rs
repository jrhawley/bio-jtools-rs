//! Filter alignments in a SAM/BAM/CRAM file.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

use bam::{Record, RecordReader, RecordWriter};

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

    // step through records and IDs
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
