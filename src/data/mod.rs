use regex::Regex;
use std::fs::{create_dir, rename, File};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::utils::detect_filetype;
use std::error::Error;
use std::io::prelude::*;

type Date = chrono::NaiveDate;

#[derive(Debug)]
struct SeqDir<'a> {
    path: &'a Path,
    date: Date,
    instrument: &'a str,
    run: u16,
    position: char,
    flowcell: &'a str,
    description: &'a str,
}

fn create_reserved_file(seq: &SeqDir, file: &str) {
    match file {
        "README.md" => create_readme(seq),
        "cluster.yaml" => create_cluster_yaml(seq),
        "Snakefile" => create_snakefile(seq),
        "config.tsv" => create_config(seq),
        _ => return
    }
    // OpenOptions::new().write(true).create_new(true).open(outfile).expect("Error creating file.");
}

fn create_reserved_dir(p: PathBuf) {
    create_dir(p).expect("Error creating directory.");
    // OpenOptions::new().write(true).create_new(true).open(outfile).expect("Error creating file.");
}

fn create_readme(seq: &SeqDir) {
    let p = seq.path.join(Path::new("README.md"));
    let mut file = match File::create(&p) {
        // The `description` method of `io::Error` returns a string that
        Err(why) => panic!("couldn't open {}: {}", p.display(), why.to_string()),
        Ok(file) => file,
    };
    let text = format!(
        "# {}\n\nFlowcell: {}\nDate Submitted: {}\nDate Received: {}\n\n## Description\n\n{}",
        seq.path.file_stem().unwrap().to_str().unwrap(),
        seq.flowcell,
        "",
        seq.date.format("%Y-%m-%d"),
        seq.description
    );
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why.to_string()),
        Ok(_) => return,
    }
}

fn create_config(seq: &SeqDir)
{
    unimplemented!();
}

fn create_cluster_yaml(seq: &SeqDir)
{
    unimplemented!();
}

fn create_snakefile(seq: &SeqDir)
{
    unimplemented!();
}

fn correct_sample_name(name: &str)
{
    unimplemented!();
}

fn mv_to_dir(file: &Path, dir: &Path) {
    rename(file, dir.join(file.file_name().unwrap())).expect("Failed to move file.");
}


pub fn organize(indir: &Path, seqtype: &str, dryrun: bool) {
    let reserved_dirnames = vec!["Reports", "FASTQs", "Trimmed", "Aligned"];
    let reserved_filenames = vec!["README.md", "cluster.yaml", "Snakefile", "config.tsv"];
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
        path: indir,
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
                create_reserved_file(&sd, f);
            } else {
                println!("  {}", f);
            }
        }
    }
    // create non-existant reserved directories
    println!("Creating directories...");
    for d in &reserved_dirnames {
        let p = indir.join(Path::new(&d));
        if !p.as_path().exists() {
            if !dryrun {
                create_reserved_dir(p);
            } else {
                println!("  {}", d);
            }
        }
    }
    // find and relocate FASTQs, if necessary
    println!("Moving sequencing files...");
    for entry in WalkDir::new(indir).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        // don't move directories
        if entry_path.is_dir() {
            continue;
        // don't move reserved file names
        } else if reserved_filenames.iter().any(|&i| i == entry_path.file_name().unwrap().to_str().unwrap()) {
            continue;
        }
        let destdir: PathBuf;
        // find out where the file needs to go
        match detect_filetype(entry_path) {
            "FASTA" | "FASTQ" => {
                destdir = indir.join(Path::new("FASTQs"));
            },
            "SAM" | "CRAM" => {
                destdir = indir.join(Path::new("Aligned"));
            }
            _ => {
                destdir = indir.join(Path::new("Reports"));
            }
        }
        if !dryrun {
            mv_to_dir(entry_path, destdir.as_path());
        }
        println!("  {} -> {}", entry_path.display(), destdir.as_path().join(entry_path.file_name().unwrap()).display());
    }
    // extract sample information from FASTQs

}
