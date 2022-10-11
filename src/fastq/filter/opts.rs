//! Options for filtering records from a FASTQ file.

use crate::{cli::CliOpt, fastq::filter::FastqFilterError, record::filter::RecordFilter};
use anyhow::bail;
use clap::Parser;
use needletail::{parser::FastqReader, FastxReader};
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
        let mut id_reader = self.get_id_file_lines()?;
        let mut fq_reader = self.get_hts_reader();
        let mut filt_iter = FastqFilterIter::new();

        let mut deal_with_remaining_reads = false;

        // writer for the output file (or STDOUT)
        let writer = self.writer_output()?;

        // initialize the iter struct
        filt_iter.get_next_id(&mut id_reader)?;
        filt_iter.get_next_record(&mut fq_reader)?;

        // this loop iterates over both files simultaneously
        while filt_iter.curr_record().is_some() && filt_iter.curr_filter_id().is_some() {
            filt_iter.assert_ids_are_sorted()?;
            filt_iter.assert_records_are_sorted()?;
            if filt_iter.curr_record_id() < filt_iter.curr_filter_id() {
                if !self.keep {
                    // unwrap is guaranteed because of the while loop condition
                    filt_iter.curr_record().unwrap().write(&mut writer, None)?;
                }

                // update the records
                filt_iter.get_next_record(&mut fq_reader);

                // check if there is a subsequent record in the FASTQ
                // TODO: Start here
                if filt_iter.curr_record().is_some() {}
            } else if filt_iter.curr_record_id() > filt_iter.curr_filter_id() {
            } else {
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
            writer.finish()?;
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
        let mut id_file = match File::open(ids) {
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
