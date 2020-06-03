use rust_lapper::{Interval, Lapper};
use std::path::Path;
// use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use prettytable::{Table, Row, Cell, row, cell};

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

pub fn multijaccard(paths: &Vec<&Path>) -> Table {
    // matrix to store pairwise results
    let mut m = Table::new();
    // add extra column and row for paths
    // create header
    let header = vec![""].iter().cloned().chain(
        paths.iter().map(|p| p.to_str().unwrap()))
        .collect::<Vec<_>>();

    m.set_titles(Row::new(header.iter().map(|p| Cell::new(p)).collect::<Vec<_>>()));
    
    for (i, p) in paths.iter().enumerate() {
        let diag = vec!["1"];
        let mut padding: Vec<&str> = vec![p.to_str().unwrap()].iter().cloned().chain(vec![""; i]).collect::<Vec<_>>();
        padding = padding.iter().cloned().chain(diag).collect::<Vec<_>>();
        let remainder: Vec<String> = paths[(i+1)..paths.len()].iter().map(|q| jaccard(p, q).2.to_string()).collect();
        let remainder_str: Vec<&str> = remainder.iter().map(|q| q.as_str()).collect();
        let entire_row: Vec<&str> = padding.into_iter().chain(remainder_str).collect();
        m.add_row(Row::new(entire_row.iter().map(|r| Cell::new(r)).collect()));
    }
    return m;
}