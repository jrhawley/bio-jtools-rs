use bam::{RecordReader, BamReader, SamReader};
use prettytable::{row, cell, format, Table};

use crate::utils::{Hts, HtsFile};

fn algn_parser<T: RecordReader>(reader: &mut T) {
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
            },
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

/// Print information about the alignment file
pub fn info(hts: &HtsFile) {
    match hts.filetype() {
        Hts::BAM => {
            let mut reader = BamReader::from_path(hts.path(), 3).unwrap();
            algn_parser(&mut reader)
        },
        Hts::SAM => {
            let mut reader = SamReader::from_path(hts.path()).unwrap();
            algn_parser(&mut reader)
        },
        _ => (),
    }
}
