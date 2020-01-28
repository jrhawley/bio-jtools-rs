use bio::io::fastq;
use crate::utils::detect_filetype;

pub fn fx_info(_file: &str) {
    let reader = fastq::Reader::from_file(_file).unwrap();
    let mut total: usize = 0;
    for _record in reader.records() {
        total += 1;
    }
    println!("{}", detect_filetype(_file));
    println!("{} reads", total);
}
