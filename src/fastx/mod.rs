use bio::io::fastq;

pub fn fx_info(_file: &str) {
    let reader = fastq::Reader::from_file(_file).unwrap();
    let mut total: usize = 0;
    for record in reader.records() {
        total += 1;
    }
    println!("{} reads", total);
}
