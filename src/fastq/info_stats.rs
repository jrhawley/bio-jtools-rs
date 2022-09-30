//! Statistics for a FASTQ file.

use crate::{
    cli::CliOpt,
    record::{
        error::RecordError,
        header::{RecordName, ILLUMINA_SEPARATOR_ASCII_CODE, RNAME_SEPARATOR_ASCII_CODE},
        stats::RecordStats,
    },
    utils::{formats::OutputFormat, Fastx, Hts, HtsFile},
};
use clap::Parser;
use needletail::parse_fastx_file;
use needletail::{errors::ParseError, parser::SequenceRecord};
use std::collections::HashMap;
use std::path::PathBuf;

/// CLI options for getting info from an HTS file
#[derive(Debug, Parser)]
pub(crate) struct FastqInfoOpts {
    /// Get info about this HTS file
    #[clap(name = "HTS")]
    hts_path: PathBuf,

    /// Count the total number of records
    #[clap(short, long)]
    total: bool,

    /// Track the frequency of sequence lengths
    #[clap(short, long)]
    lengths: bool,

    /// Track the sequencing instruments used (only valid for FASTQ, SAM, BAM, and CRAM files)
    #[clap(short, long)]
    instruments: bool,

    /// Track flow cell IDs
    #[clap(short = 'F', long)]
    flow_cell_ids: bool,

    /// Output format to return statistics in
    #[clap(short = 'f', long, default_value = "human")]
    format: OutputFormat,

    /// Keep statistics on the first N records
    #[clap(short = 'N', long = "max-records", value_name = "N")]
    n_max_records: Option<u64>,
}

impl FastqInfoOpts {
    /// Get information and statistics about a desired FASTQ file
    fn calc_fastq_info(&self, hts: HtsFile) -> FastqStats {
        let mut stats = FastqStats::new();
        let mut reader = parse_fastx_file(hts.path()).expect("Error opening HTS file");

        if let Some(n_max) = self.n_max_records {
            // check if the max capacity has been hit
            while let (true, Some(record)) = (stats.n_records() < n_max, reader.next()) {
                stats.process_record(&record, self);
            }
        } else {
            while let Some(record) = reader.next() {
                stats.process_record(&record, self);
            }
        }

        stats
    }
}

impl CliOpt for FastqInfoOpts {
    fn exec(&self) -> anyhow::Result<()> {
        let hts = HtsFile::new(&self.hts_path);
        match hts.filetype() {
            Hts::Fastx(Fastx::Fastq) => {
                let stats = self.calc_fastq_info(hts);
                println!("{:#?}", stats);
            }
            _ => todo!(),
        }

        Ok(())
    }
}

/// Statistics from a FASTQ file
#[derive(Debug)]
pub(crate) struct FastqStats {
    /// Total number of valid records
    valid_records: u64,

    /// Total number of invalid records
    invalid_records: u64,

    /// Total number of bases in a file
    bases: u64,

    /// Length distribution of records
    lengths: HashMap<u64, u64>,

    /// Sequencing instruments
    instruments: HashMap<String, u64>,

    /// Flow cell IDs
    flow_cell_ids: HashMap<String, u64>,
}

impl FastqStats {
    /// Process an Illumina (Casava >= v1.8) formatted FASTQ record
    fn process_illumina_split_record(&mut self, rname: &[u8], opts: &FastqInfoOpts) {
        // Illumina Casava >= v1.8 format
        let mut id_splits = rname.split(|x| *x == ILLUMINA_SEPARATOR_ASCII_CODE);

        // instrument name
        let inst = id_splits.next();
        if opts.instruments {
            self.process_illumina_instrument(inst);
        }

        // run ID
        let run_id = id_splits.next();

        // flow cell ID
        let fcid = id_splits.next();
        if opts.flow_cell_ids {
            self.process_illumina_flowcell(fcid);
        }
    }
}

impl<'a> RecordStats<'a> for FastqStats {
    type Record = SequenceRecord<'a>;
    type Error = ParseError;
    type InfoOpts = FastqInfoOpts;

    /// Create a new set of statistics for a FASTQ file
    fn new() -> Self {
        FastqStats {
            valid_records: 0,
            invalid_records: 0,
            bases: 0,
            lengths: HashMap::new(),
            instruments: HashMap::new(),
            flow_cell_ids: HashMap::new(),
        }
    }

    fn n_valid(&self) -> u64 {
        self.valid_records
    }

    fn n_invalid(&self) -> u64 {
        self.invalid_records
    }

    fn mut_lengths(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.lengths
    }

    fn mut_flow_cells(&mut self) -> &mut HashMap<String, u64> {
        &mut self.flow_cell_ids
    }

    fn mut_instruments(&mut self) -> &mut HashMap<String, u64> {
        &mut self.instruments
    }

    fn process_valid_record(&mut self, seq: &SequenceRecord, opts: &FastqInfoOpts) {
        self.valid_records += 1;

        let seq_length: u64 = seq.num_bases().try_into().unwrap();
        self.bases += seq_length;

        if opts.lengths {
            self.update_lengths(seq_length);
        }
        if opts.flow_cell_ids || opts.instruments {
            if opts.flow_cell_ids || opts.instruments {
                match RecordName::try_from(seq.id()) {
                    Ok(RecordName::CasavaV1_8) => {
                        let mut splits = seq.id().split(|x| *x == RNAME_SEPARATOR_ASCII_CODE);
                        let a = splits.next().unwrap();
                        self.process_illumina_split_record(a, opts);
                    }
                    Ok(RecordName::SequenceReadArchive) => {
                        self.process_sra_split_record();
                    }
                    Err(RecordError::UncertainRecordNameFormat) => todo!(),
                }
            }
        }
    }

    fn process_invalid_record(&mut self) {
        self.invalid_records += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let expected = 4;
        let observed = 2 + 2;

        assert_eq!(expected, observed);
    }
}
