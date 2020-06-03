use rust_lapper::{Interval, Lapper};
use std::path::Path;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

type Iv = Interval<u32>;

fn line_to_intvl(line: Result<String, io::Error>) -> Iv {
    let l = line.unwrap();
    let mut tabsplit = l.split(|c| c == '\t');
    let chrom = tabsplit.next().unwrap();
    let start: u32 = tabsplit.next().unwrap().parse::<u32>().unwrap();
    let end: u32 = tabsplit.next().unwrap().parse::<u32>().unwrap();
    Interval{start: start, stop: end, val: 0}
}

pub fn jaccard(a: &Path, b: &Path) -> (u32, u32, f64) {
    // naive implementation: load both files into memory and intersect them
    let file_a = File::open(a).unwrap();
    let file_b = File::open(b).unwrap();
    let data_a: Vec<Iv> = io::BufReader::new(file_a).lines().map(|l| line_to_intvl(l)).collect();
    let data_b: Vec<Iv> = io::BufReader::new(file_b).lines().map(|l| line_to_intvl(l)).collect();

    let lap_a = Lapper::new(data_a);
    let lap_b = Lapper::new(data_b);
    let (union, intersect) = lap_a.union_and_intersect(&lap_b);
    let j = f64::from(intersect) / f64::from(union);
    return (intersect, union, j);
}

// fn jaccard(a: IntervalTree, b: IntervalTree) -> f32
// {
//     return 0.0f32;
// }

// pub fn multijaccard()
// {
// }