//! # Processing BED and other interval-based files
//! Functions and methods related to processing files based on genomic intervals, such as [BED](https://bedtools.readthedocs.io/en/latest/content/general-usage.html) files and its variants.

use itertools::Itertools;
use prettytable::{format, Cell, Row, Table};
use rust_lapper::{Interval, Lapper};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::utils::HtsFile;

type Iv = Interval<u32>;

fn line_to_intvl(line: Result<String, io::Error>) -> (String, Iv) {
    let l = line.unwrap();
    let mut tabsplit = l.split(|c| c == '\t');
    let chrom = tabsplit.next().unwrap();
    let start: u32 = tabsplit.next().unwrap().parse::<u32>().unwrap();
    let end: u32 = tabsplit.next().unwrap().parse::<u32>().unwrap();
    return (
        chrom.to_string(),
        Interval {
            start: start,
            stop: end,
            val: 0,
        },
    );
}

fn file_to_chromlap(file: File) -> HashMap<String, Lapper<u32>> {
    let mut file_data: HashMap<String, Vec<Iv>> = HashMap::new();

    // iterate over file lines
    for l in io::BufReader::new(file).lines() {
        // create interval from the line
        let (chr, iv) = line_to_intvl(l);
        // store it in the vector
        if let Some(x) = file_data.get_mut(&chr) {
            x.push(iv);
        } else {
            let blank = vec![iv];
            file_data.insert(chr, blank);
        }
    }

    // convert Vec into single Lapper objects
    let mut lap: HashMap<String, Lapper<u32>> = HashMap::new();
    for chrom in file_data.keys() {
        lap.insert(chrom.to_string(), Lapper::new(file_data[chrom].to_vec()));
    }

    return lap;
}

pub fn jaccard(a: &HtsFile, b: &HtsFile) -> (u32, u32, f64) {
    // naive implementation: load both files into memory and intersect them
    let file_a = File::open(a.path()).unwrap();
    let file_b = File::open(b.path()).unwrap();
    // create HashMap of the data, by chromosome
    let lap_a = file_to_chromlap(file_a);
    let lap_b = file_to_chromlap(file_b);

    // iterate over all chromosomes to calculate intersections/unions per chromosome
    let mut union: u32 = 0;
    let mut intersect: u32 = 0;
    for chrom in lap_a.keys().chain(lap_b.keys()).unique() {
        let l_a: &Lapper<u32>;
        let l_b: &Lapper<u32>;
        let blank = Lapper::new(vec![Interval {
            start: 0,
            stop: 0,
            val: 0,
        }]); // temporary empty lapper

        // chrom will be in one of a or b, check if it's not in one of them
        if !lap_a.contains_key(chrom) {
            l_a = &blank;
            l_b = &lap_b[chrom];
        } else if !lap_b.contains_key(chrom) {
            l_a = &lap_a[chrom];
            l_b = &blank;
        } else {
            l_a = &lap_a[chrom];
            l_b = &lap_b[chrom];
        }

        // calculate union and intersection for this chromosome
        let (u, i) = l_a.union_and_intersect(l_b);
        // add to total counts across all chromosomes
        union += u;
        intersect += i;
    }
    let j = f64::from(intersect) / f64::from(union);
    return (intersect, union, j);
}

pub fn multijaccard(files: &Vec<&HtsFile>) -> Table {
    // matrix to store pairwise results
    let mut m = Table::new();
    m.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    // add extra column and row for paths
    // create header
    let header = vec![""]
        .iter()
        .cloned()
        .chain(files.iter().map(|h| h.path().to_str().unwrap()))
        .collect::<Vec<_>>();

    // set titles of table as filenames of provided BED files
    m.set_titles(Row::new(
        header.iter().map(|p| Cell::new(p)).collect::<Vec<_>>(),
    ));
    // iterate over pairs of BED files
    for (i, p) in files.iter().enumerate() {
        // skip self-similar calculations
        let diag = vec!["1"];
        // empty spaces in the table to only calculate pairs once
        let mut padding: Vec<&str> = vec![p.path().to_str().unwrap()]
            .iter()
            .cloned()
            .chain(vec![""; i])
            .collect::<Vec<_>>();
        padding = padding.iter().cloned().chain(diag).collect::<Vec<_>>();
        // calculate pairwise Jaccard indices for each remaining pair of files
        let remainder: Vec<String> = files[(i + 1)..files.len()]
            .iter()
            .map(|q| jaccard(p, q).2.to_string())
            .collect();
        // convert the values in a Vec, append to padding, then make into a table row
        let remainder_str: Vec<&str> = remainder.iter().map(|q| q.as_str()).collect();
        let entire_row: Vec<&str> = padding.into_iter().chain(remainder_str).collect();
        m.add_row(Row::new(entire_row.iter().map(|r| Cell::new(r)).collect()));
    }
    // return the table for printing or saving
    return m;
}
