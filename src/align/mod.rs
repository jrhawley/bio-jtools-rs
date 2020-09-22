use bam::{RecordReader, BamReader, SamReader};

use crate::utils::HtsFile;

fn get_reader(hts: &HtsFile) -> Box<dyn RecordReader> {
    match hts.is_zipped() {
        // create a reader object that parses the file with 4 threads
        true => Box::new(BamReader::from_path(hts.path(), 4).unwrap()),
        // create a reader object that parses the file with a single thread if unzipped
        false => Box::new(SamReader::from_path(hts.path()).unwrap()),
    }
}

/// Print information about the alignment file
pub fn info(hts: &HtsFile) {
    let mut reader: Box<dyn RecordReader> = get_reader(hts);

    // values to keep and display
    let mut n_records: u32 = 0; // number of records
    let mut n_bases: u32 = 0; // number of bases
    let mut n_errs: u32 = 0; // number of alignments resulting in parse errors

    let mut record = bam::Record::new();

    // parse the alignment file
    loop {
        match (*reader).read_into(&mut record) {
            Ok(false) => n_errs +=1,
            Ok(true) => {
                // add to n_records count
                n_records += 1;
                // keep track of the n_records number of bases
                n_bases += record.sequence().len() as u32;
            },
            Err(e) => panic!("{}", e),
        }
    }

    // print output
    println!("{} bases", n_bases);
    println!("{} reads", n_records);
    if n_errs > 0 {
        println!("{} alignments containing errors", n_errs);
    }
}
