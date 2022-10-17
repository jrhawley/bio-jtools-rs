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
        self.filter_with_id_file()
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
        // get the first filtering ID
        filt_iter.get_next_id(&mut id_reader)?;

        // writer for the output file (or STDOUT)
        let mut writer = self.writer_output()?;

        // each iteration of this loop will process a record from the FASTQ
        while let Some(possible_rec) = fq_reader.next() {
            // bail early if there is an error parsing the record
            match possible_rec {
                Err(e) => bail!(FastqFilterError::CannotParseFastqRecord(e)),
                Ok(rec) => {
                    // update the previous record ID
                    filt_iter.set_prev_record_id(&rec)?;

                    // ensure that the FASTQ records are sorted
                    if let Some(prev_rec_id) = filt_iter.prev_record_id() {
                        if rec.id() < prev_rec_id {
                            bail!(FastqFilterError::FastqNotSorted);
                        }
                    }

                    // check the record ID against the previous and current filtering IDs
                    // we need this inner loop to iterate through the filtering IDs before
                    // moving on to the next FASTQ record
                    while let Some(filter_id) = filt_iter.curr_filter_id() {
                        if rec.id() > filter_id {
                            // advanced the IDs and try again
                            filt_iter.get_next_id(&mut id_reader)?;
                        } else {
                            break;
                        }
                    }

                    if let Some(filter_id) = filt_iter.curr_filter_id() {
                        // now guaranteed that current filter ID is >= record ID
                        if rec.id() == filter_id && self.keep {
                            rec.write(&mut writer, None)?;
                        } else if !self.keep {
                            rec.write(&mut writer, None)?;
                        }
                    } else {
                        if !self.keep {
                            rec.write(&mut writer, None)?;
                        }
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let fq = b"@First\nACGT\n+\nFFFF";
        let mut fq_reader = FastqReader::new(&fq[..]);
        let header = String::from_utf8(fq_reader.next().unwrap().unwrap().id().to_vec()).unwrap();
        assert_eq!("", &header);
    }
}
