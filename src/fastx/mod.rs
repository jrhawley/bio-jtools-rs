use bio::io::fastq;
use crate::utils::detect_filetype;

// fn fx_header_type(_header: &str) -> &str
// {
//     return "";
// }

pub fn fx_info(_path: &str)
{
    let reader = fastq::Reader::from_file(_path).unwrap();
    let mut total: usize = 0;
    for _record in reader.records() {
        let record = _record.unwrap_or_else(|_| fastq::Record::new());
        if !record.is_empty() {
            total += 1;
        }
    }
    println!("{}", detect_filetype(_path));
    println!("{} reads", total);
}
