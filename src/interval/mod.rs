// use bio::data_structures::interval_tree::IntervalTree;
// use bio::utils::Interval;
use bio::io::bed;

pub fn jaccard_path(_a: &str, _b: &str)
{
    let mut reader_a = bed::Reader::from_file(_a).expect("Error reading file.");
    let mut reader_b = bed::Reader::from_file(_b).expect("Error reading file.");
    let mut records_a = reader_a.records();
    let mut records_b = reader_b.records();
    // Assuming sorted, starting with first record in each file
    let mut rec_a = records_a.next().unwrap().ok().expect("Error reading record.");
    let mut rec_b = records_b.next().unwrap().ok().expect("Error reading record.");
    // loop {
    //     println!("{}\t{}\t{}", rec_a.chrom(), rec_a.start(), rec_a.end());
    //     if rec_a.chrom() > rec_b.chrom() {
    //         rec_b = records_b.next().unwrap().ok().expect("Error reading record.");
    //     } else {
    //         rec_a = records_a.next().unwrap().ok().expect("Error reading record.");
    //     }
    //     break;
    // }
    // return IntervalTree::new();
}

// fn jaccard(a: IntervalTree, b: IntervalTree) -> f32
// {
//     return 0.0f32;
// }

// pub fn multijaccard(beds: [&str])
// {
// }