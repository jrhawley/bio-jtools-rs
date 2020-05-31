use regex::Regex;
use std::fs::{OpenOptions, create_dir};
use std::path::Path;

type Date = chrono::NaiveDate;

#[derive(Debug)]
struct SeqDir<'a> {
    date: Date,
    instrument: &'a str,
    run: u16,
    position: char,
    flowcell: &'a str,
    description: &'a str,
}

fn create_readme(seq: SeqDir, outfile: &str)
{
    unimplemented!();
}

fn create_config(seq: SeqDir, outfile: &str)
{
    unimplemented!();
}

fn create_cluster_yaml(seq: SeqDir, outfile: &str)
{
    unimplemented!();
}

fn create_snakefile(seq: SeqDir, outfile: &str)
{
    unimplemented!();
}

fn correct_sample_name(name: &str)
{
    unimplemented!();
}


pub fn organize(indir: &Path, seqtype: &str, dryrun: bool) {
    let reserved_dirnames = vec!["Reports", "FASTQs", "Trimmed", "Aligned"];
    let reserved_filenames = vec!["README.md", "cluster.yaml", "Snakefile", "setup.log", "config.tsv"];
    let fq_regex = Regex::new(r"^([A-Za-z0-9-_]+)_S([1-9][0-9]?)_L00(\d)_(I[1-3]|R[1-3])_001\.f(ast)?q(\.gz)?$").unwrap();
    let dir_regex = Regex::new(r"^([0-9]{2})(0?[1-9]|1[012])(0[1-9]|[12]\d|3[01])_(\w{3,})_(\d{4})_(A|B)(\w{9})(.*)?").unwrap();
    let dir_stem = indir.file_stem().unwrap();
    // extract flowcell information from directory name
    let cap = dir_regex.captures(dir_stem.to_str().unwrap()).unwrap();
    let date = Date::parse_from_str(
        &format!("{}{}{}", cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
        "%y%m%d"
    ).unwrap();
    let sd = SeqDir{
        date: date,
        instrument: cap.get(4).unwrap().as_str(),
        run: cap.get(5).unwrap().as_str().parse::<u16>().unwrap(),
        position: cap.get(6).unwrap().as_str().parse::<char>().unwrap(),
        flowcell: cap.get(7).unwrap().as_str(),
        description: cap.get(8).unwrap().as_str(),
    };
    // create non-existant reserved files
    println!("Creating files...");
    for f in &reserved_filenames {
        let p = indir.join(Path::new(&f));
        if !p.as_path().exists() {
            if !dryrun {
                OpenOptions::new().write(true).create_new(true).open(p).expect("Error creating file.");
            } else {
                println!("{}", p.as_path().display());
            }
        }
    }
    // create non-existant reserved directories
    println!("Creating directories...");
    for d in &reserved_dirnames {
        let p = indir.join(Path::new(&d));
        if !p.as_path().exists() {
            if !dryrun {
                create_dir(p).expect("Error creating directory.");
            } else {
                println!("{}", p.as_path().display());
            }
        }
    }
    // find and relocate FASTQs, if necessary
    println!("Moving sequencing files...");
    // extract sample information from FASTQs

}
