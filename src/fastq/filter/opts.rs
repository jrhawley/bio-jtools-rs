//! Options for filtering records from a FASTQ file.

use crate::{cli::CliOpt, fastq::filter::FastqFilterError, record::filter::RecordFilter};
use anyhow::bail;
use clap::Parser;
use needletail::{
    errors::ParseError,
    parser::{FastqReader, SequenceRecord},
    FastxReader,
};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use super::iter::FastqFilterIter;

/// Options for filtering reads from a FASTQ file.
#[derive(Debug, Parser)]
pub struct FastqFilterOpts {
    /// Get info about this HTS file.
    #[clap(name = "HTS")]
    hts_path: PathBuf,

    /// Regular expression to match against the read names.
    #[clap(short, long, conflicts_with = "id_list_path")]
    regex: Option<Regex>,

    /// Text file containing all read names to filter.
    #[clap(
        short = 'f',
        long = "id-file",
        value_name = "FILE",
        conflicts_with = "regex"
    )]
    id_list_path: Option<PathBuf>,

    /// Output file name.
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Keep the records that match, instead of discarding them.
    #[clap(short, long)]
    keep: bool,
}

impl CliOpt for FastqFilterOpts {
    fn exec(&self) -> anyhow::Result<()> {
        match (self.regex.is_some(), self.id_list_path.is_some()) {
            (true, true) => {
                // this should be excluded by the CLI
                bail!(FastqFilterError::CannotSpecifyRegexAndIdFile)
            }
            (true, false) => self.filter_with_id_regex(),
            (false, true) => self.filter_with_id_file(),
            (false, false) => bail!(FastqFilterError::FilterCannotBeEmpty),
        }
    }
}

impl RecordFilter for FastqFilterOpts {
    fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}

impl FastqFilterOpts {
    /// Filter out records using a sorted ID file to match against.
    fn filter_with_id_file(&self) -> anyhow::Result<()> {
        let n_records_processed = 0u32;
        let mut id_reader = self.get_id_file_lines()?;
        let mut fq_reader = self.get_hts_reader();
        let mut filt_iter = FastqFilterIter::new();
        // writer for the output file (or STDOUT)
        let mut writer = self.writer_output()?;
        let mut deal_with_remaining_reads = false;

        // The current record being processed.
        // This belongs in this scope instead of inside `filt_iter` because of
        // mutable references to data owned by `fq_reader`.
        let mut curr_record = fq_reader.next();

        // this loop iterates over both files simultaneously
        loop {
            let mut update_record = false;
            let mut update_id = false;

            assert_records_are_sorted(&curr_record, filt_iter.prev_record_id())?;

            if let (Some(Ok(rec)), Some(filter_id)) = (curr_record, filt_iter.curr_filter_id()) {
                if rec.id() < filter_id {
                    // if the record is before the filtering ID and we are not keeping the filtering IDs
                    // then we should write it to the output
                    if !self.keep {
                        rec.write(&mut writer, None)?;
                    }

                    // update the records
                    update_record = true;
                } else if rec.id() > filter_id {
                    // update the IDs
                    update_id = true;
                } else {
                    if self.keep {
                        // unwrap is guaranteed because of the while loop condition
                        rec.write(&mut writer, None)?;
                    }

                    // update the records
                    update_record = true;
                }
            }

            // if update_record {
            //     filt_iter.set_prev_record_id(&curr_record)?;
            //     curr_record = fq_reader.next();
            // } else if update_id {
            //     // if the record is after the filtering ID, don't do anything with the current record right away
            //     filt_iter.get_next_id(&mut id_reader)?;

            //     // if there are no more IDs to process, be sure to deal with the remaining records
            //     // outside of the present loop
            //     if filt_iter.curr_filter_id().is_none() {
            //         // unwrap is guaranteed because of the while loop condition
            //         curr_record.unwrap().unwrap().write(&mut writer, None)?;
            //         deal_with_remaining_reads = true;
            //     }
            // }

            // if the updated current record or current filtering ID is `None`, then our comparisons are done
            // and we can break out of the look
            if curr_record.is_none() || filt_iter.curr_filter_id().is_none() {
                break;
            }
        }

        // If all of the IDs have been exhausted but there are still records to write,,
        // write them without comparing against the IDs.
        // We can assume this because the FASTQ and ID file are both sorted.
        if deal_with_remaining_reads && !self.keep {
            while let Some(Ok(record)) = fq_reader.next() {
                record.write(&mut writer, None)?;
            }
        }

        Ok(())
    }

    pub fn get_hts_reader(&self) -> FastqReader<File> {
        FastqReader::from_path(&self.hts_path).expect("Error opening HTS file.")
    }

    pub fn get_id_file_lines(&self) -> Result<io::Lines<BufReader<File>>, FastqFilterError> {
        // the unwrap is guaranteed based on the `FastqFilterOpts::exec()` implementation
        let ids = match self.id_list_path.as_ref() {
            Some(path) => path,
            None => return Err(FastqFilterError::IdFileNotProvidedWhenRequired),
        };

        // open the file with the IDs to filter
        let id_file = match File::open(ids) {
            Ok(f) => BufReader::new(f).lines(),
            Err(_) => return Err(FastqFilterError::IdFileCannotBeOpened),
        };

        Ok(id_file)
    }

    /// Filter out records using a regular expression to match against record IDs.
    fn filter_with_id_regex(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Filter out records containing a provided sequence.
    fn filter_seq(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Filter out records based on its quality scores.
    fn filter_qual(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Check that the FASTQ records are in sorted order.
fn assert_records_are_sorted(
    curr_record: &Option<Result<SequenceRecord, ParseError>>,
    prev_record_id: Option<&[u8]>,
) -> Result<(), FastqFilterError> {
    match (curr_record, prev_record_id) {
        (Some(Ok(curr)), Some(prev)) => {
            if curr.id() < prev {
                Ok(())
            } else {
                Err(FastqFilterError::FastqNotSorted)
            }
        }
        (Some(Err(e)), _) => Err(FastqFilterError::CannotParseFastqRecord(e.clone())),
        (_, _) => Ok(()),
    }
}
