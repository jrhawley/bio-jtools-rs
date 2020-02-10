// use bio::io::fastq;
use crate::utils::detect_filetype;
use fastq::parse_path;

// fn fx_header_type(_header: &str) -> &str
// {
//     return "";
// }

pub fn fx_info(_path: &str)
{
    let mut total: usize = 0;
    parse_path(Some(_path), |parser| {
        let stopped = parser.each(|_| {total += 1; true})
    }).expect("Invalid FASTQ file");
    println!("{}", detect_filetype(_path));
    println!("{} reads", total);
}
