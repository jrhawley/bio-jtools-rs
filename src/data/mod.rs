use indoc::indoc;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::{create_dir, rename, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::{Hts, HtsFile, Fastx};

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

#[derive(Debug)]
struct SeqSample {
    sample: String,
    index: u8,
    mates: Vec<String>,
    lanes: Vec<String>,
}
impl fmt::Display for SeqSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t",
            self.sample,
            self.index,
            self.mates.join(","),
            self.lanes.join(","),
        )
    }
}

fn create_reserved_file(seq: &SeqDir, file: &str) {
    match file {
        "README.md" => create_readme(seq),
        "cluster.yaml" => create_cluster_yaml(seq),
        "Snakefile" => create_snakefile(seq),
        // exclude config.tsv, make that file separately when you reorganize the FASTQs
        _ => return,
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

fn create_cluster_yaml(seq: &SeqDir) {
    let p = seq.path.join(Path::new("cluster.yaml"));
    let mut file = match File::create(&p) {
        // The `description` method of `io::Error` returns a string that
        Err(why) => panic!("couldn't open {}: {}", p.display(), why.to_string()),
        Ok(file) => file,
    };
    let text = indoc!(
        "__default__:
            params: '-p all --export=ALL -t 1-00:00:00 --mem 6G'
        "
    );
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why.to_string()),
        Ok(_) => return,
    }
}

fn create_snakefile(seq: &SeqDir) {
    let p = seq.path.join(Path::new("Snakefile"));
    let mut file = match File::create(&p) {
        // The `description` method of `io::Error` returns a string that
        Err(why) => panic!("couldn't open {}: {}", p.display(), why.to_string()),
        Ok(file) => file,
    };
    let text = indoc!(
        "
        # =============================================================================
        # Configuration
        # =============================================================================
        import pandas as pd
        import os.path as path

        CONFIG = pd.read_csv('config.tsv', index_col=False, sep='\\t')
        CONFIG = CONFIG.loc[~CONFIG.Sample.str.startswith('#'), :]

        REPORT_DIR = 'Reports'
        FASTQ_DIR = 'FASTQs'
        ALIGN_DIR = 'Aligned'

        SAMPLES = CONFIG['Sample'].tolist()
        READS = [1, 2]
        LANES = [1, 2, 3, 4]

        BWT2_IDX = '/path/to/data/genomes/human/hg38/iGenomes/Sequence/Bowtie2Index/genome'
        CHRS = ['chr' + str(i) for i in list(range(1, 23)) + ['X', 'Y']]

        wildcard_constraints:
            sample = '[A-Za-z0-9-]+',
            lane = '[1-4]',
            read = '[1-2]'

        # =============================================================================
        # Meta Rules
        # =============================================================================
        rule all:
            input:
                path.join(REPORT_DIR, 'multiqc_report.html'),
                expand(
                    path.join(REPORT_DIR, '{{sample}}_L00{{lane}}_R{{read}}_fastqc.zip'),
                    sample=SAMPLES,
                    lane=LANES,
                    read=READS
                ),

        rule rulegraph:
            output:
                'rulegraph.png',
            shell:
                'snakemake --rulegraph | dot -Tpng > {{output}}'

        # =============================================================================
        # Rules
        # =============================================================================
        # Summaries
        # -----------------------------------------------------------------------------
        rule fastqc:
            input:
                path.join(FASTQ_DIR, '{{file}}.fastq.gz')
            output:
                path.join(REPORT_DIR, '{{file}}_fastqc.html'),
                path.join(REPORT_DIR, '{{file}}_fastqc.zip')
            params:
                '-o {{}}'.format(REPORT_DIR)
            shell:
                'fastqc {{params}} {{input}}'
        rule multiqc:
            input:
                samples = expand(
                    path.join(REPORT_DIR, '{{sample}}_fastqc.zip'),
                    sample=SAMPLES
                )
            output:
                path.join(REPORT_DIR, 'multiqc_report.html')
            shell:
                'multiqc -f -o {{REPORT_DIR}} {{REPORT_DIR}}'

        # Miscellaneous
        # -----------------------------------------------------------------------------
        rule sort_bam_name:
            input:
                '{{file}}.bam'
            output:
                '{{file}}.name-sorted.bam',
            shell:
                'sambamba sort -t 8 --tmpdir . -n -p -o {{output}} {{input}}'

        rule sort_bam:
            input:
                '{{file}}.bam'
            output:
                bam = '{{file}}.sorted.bam',
                idx = '{{file}}.sorted.bam.bai'
            shell:
                'sambamba sort -t 8 --tmpdir . -p {{input}}'
        "
    );
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why.to_string()),
        Ok(_) => return,
    }
}

fn update_sample(s: &mut SeqSample, mate: String, lane: String) {
    if !s.mates.contains(&mate) {
        s.mates.push(mate);
    }
    if !s.lanes.contains(&lane) {
        s.lanes.push(lane);
    }
}

fn create_config(seq: &SeqDir) {
    // return if the config already exists
    if seq.path.join(Path::new("config.tsv")).exists() {
        return;
    }
    // sample name + optional sample index + optional lane + optional read mate/index number + optional _001 suffix
    // this produces the following captures:
    // 1. name
    // 2. sample index
    // 3. lane index
    // 4. mate/index
    let fq_regex = Regex::new(
        r"^([A-Za-z0-9-]+)(?:_S([1-9][0-9]?))?(?:_L00(\d))?(?:_(I[1-3]|R[1-3]))?(?:_001)?\.f(?:ast)?q(?:\.gz)?$",
    )
    .unwrap();
    let mut samples = HashMap::<String, SeqSample>::new();
    for entry in WalkDir::new(seq.path.join(Path::new("FASTQs")))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let hts_file = HtsFile::new(entry.path());
        // don't move directories, only assess FASTQs
        match hts_file.filetype() {
            Hts::FASTX(Fastx::FASTQ) => {
                let fname = hts_file.path().file_name().unwrap().to_str().unwrap();
                let cap = fq_regex.captures(fname);
                // deal with the capture
                match cap {
                    Some(c) => {
                        let sample = c.get(1).unwrap().as_str().to_owned();
                        let index = match c.get(2) {
                            Some(i) => i.as_str().parse::<u8>().unwrap(),
                            None => 0,
                        };
                        let lane = match c.get(3) {
                            Some(l) => l.as_str().to_string(),
                            None => "".to_string(),
                        };
                        let mate = match c.get(4) {
                            Some(m) => m.as_str().to_string(),
                            None => "".to_string(),
                        };
                        //check for existing samples of the same name, and add new information
                        if samples.contains_key(&sample) {
                            samples
                                .entry(sample)
                                .and_modify(|e| update_sample(e, mate, lane));
                        } else {
                            let new_sample = SeqSample {
                                // create a separate copy of this string
                                sample: sample.to_string(),
                                index: index,
                                mates: vec![mate],
                                lanes: vec![lane],
                            };
                            samples.insert(sample, new_sample);
                        }
                    },
                    None => continue,
                }
            },
            _ => {},
        }
    }
    // write sample information to config.tsv
    let p = seq.path.join(Path::new("config.tsv"));
    let mut file = match File::create(&p) {
        // The `description` method of `io::Error` returns a string that
        Err(why) => panic!("couldn't open {}: {}", p.display(), why.to_string()),
        Ok(file) => file,
    };
    let mut text = "Sample_ID\tSample_Index\tMates\tLanes\tDescription\n".to_string();
    // append new row for each sample
    for (_, s) in &samples {
        text.push_str(&format!("{}", s));
    }
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why.to_string()),
        Ok(_) => return,
    }
}

fn mv_to_dir(file: &Path, dir: &Path) {
    rename(file, dir.join(file.file_name().unwrap())).expect("Failed to move file.");
}

pub fn organize(indir: &Path, dryrun: bool, verbose: bool) {
    let reserved_dirnames = vec!["Reports", "FASTQs", "Trimmed", "Aligned"];
    let reserved_filenames = vec!["README.md", "Snakefile", "cluster.yaml", "config.tsv"];
    let dir_regex = Regex::new(
        r"^([0-9]{2})(0?[1-9]|1[012])(0[1-9]|[12]\d|3[01])_(\w{3,})_(\d{4})_(A|B)(\w{9})(.*)?",
    )
    .unwrap();
    let dir_stem = indir.file_stem().unwrap();
    // extract flowcell information from directory name
    let cap = dir_regex.captures(dir_stem.to_str().unwrap()).unwrap();
    let date = Date::parse_from_str(
        &format!(
            "{}{}{}",
            cap.get(1).unwrap().as_str(),
            cap.get(2).unwrap().as_str(),
            cap.get(3).unwrap().as_str()
        ),
        "%y%m%d",
    )
    .unwrap();
    let sd = SeqDir {
        path: indir,
        date: date,
        instrument: cap.get(4).unwrap().as_str(),
        run: cap.get(5).unwrap().as_str().parse::<u16>().unwrap(),
        position: cap.get(6).unwrap().as_str().parse::<char>().unwrap(),
        flowcell: cap.get(7).unwrap().as_str(),
        description: cap.get(8).unwrap().as_str(),
    };
    // create non-existant reserved files
    if verbose {
        println!("Creating files...");
    }
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
    if verbose {
        println!("Creating directories...");
    }
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
    if verbose {
        println!("Moving sequencing files...");
    }
    for entry in WalkDir::new(indir).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        // don't move directories
        if entry_path.is_dir() {
            continue;
        // don't move reserved file names
        } else if reserved_filenames
            .iter()
            .any(|&i| i == entry_path.file_name().unwrap().to_str().unwrap())
        {
            continue;
        }

        let hts_file = HtsFile::new(entry_path);
        let destdir: PathBuf;
        // find out where the file needs to go
        match hts_file.filetype() {
            Hts::FASTX(_) => {
                destdir = indir.join(Path::new("FASTQs"));
            }
            Hts::SAM | Hts::CRAM => {
                destdir = indir.join(Path::new("Aligned"));
            }
            _ => {
                destdir = indir.join(Path::new("Reports"));
            }
        }
        if !dryrun {
            mv_to_dir(entry_path, destdir.as_path());
        }
        if verbose {
            println!(
                "  {} -> {}",
                entry_path.display(),
                destdir
                    .as_path()
                    .join(entry_path.file_name().unwrap())
                    .display()
            );
        }
    }
    // extract sample information from FASTQs, reorganize
    if verbose {
        println!("Extracting sample information...");
    }
    create_config(&sd);
    if verbose {
        println!("Done.");
    }
}
