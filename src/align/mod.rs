use rust_htslib::{bam, bam::Read};
use std::path::Path;

pub fn info(path: &Path) {
    let mut bam = bam::Reader::from_path(path).unwrap();
    for r in bam.records() {
        let record = r.unwrap();
        println!("{:?}", record.mapq());
    }
}