use indoc::indoc;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::{create_dir, rename, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::Local;

use crate::utils::{Hts, HtsFile, Fastx, detect_filetype};

type Date = chrono::NaiveDate;

const RESERVED_DIRNAMES: [&'static str; 7] = ["Reports", "FASTQs", "Trimmed", "Aligned", "Peaks", "Variants", "Logs"];
const RESERVED_FILENAMES: [&'static str; 5]= ["README.md", "Snakefile", "cluster.yaml", "config.tsv", "setup.log"];

#[derive(Debug)]
struct SeqDir {
    path: PathBuf,
    date: Date,
    instrument: String,
    run: u8,
    position: char,
    flowcell: String,
    description: String,
}

impl SeqDir {
    /// Construct a new SeqDir object from a path
    pub fn new(path: &Path) -> SeqDir {
        let dir_regex: Regex = Regex::new(r"^([0-9]{2})(0?[1-9]|1[012])(0[1-9]|[12]\d|3[01])_(\w{3,})_(\d{4})_(A|B)(\w{9})(.*)?").unwrap();
        // if given a relative path, force it to an absolute path
        let dir_stem = match path.is_relative() {
            true => String::from(path.canonicalize().unwrap().file_stem().unwrap().to_str().unwrap()),
            false => String::from(path.file_stem().unwrap().to_str().unwrap()),
        };
        // try extracting flowcell information from directory name
        let dir_stem_capture_attempt = dir_regex.captures(&dir_stem);
        match dir_stem_capture_attempt {
            Some(cap) => {
                // extract sequencing date
                let date = Date::parse_from_str(
                    &format!(
                        "{}{}{}",
                        cap.get(1).unwrap().as_str(),
                        cap.get(2).unwrap().as_str(),
                        cap.get(3).unwrap().as_str()
                    ),
                    "%y%m%d",
                ).unwrap();
                let instrument = cap.get(4).unwrap().as_str();
                let run = cap.get(5).unwrap().as_str().parse::<u8>().unwrap();
                let pos = cap.get(6).unwrap().as_str().parse::<char>().unwrap();
                let flowcell = cap.get(7).unwrap().as_str();
                let description = cap.get(8).unwrap().as_str();
                
                SeqDir {
                    path: path.to_path_buf(),
                    date: date,
                    instrument: String::from(instrument),
                    run: run,
                    position: pos,
                    flowcell: String::from(flowcell),
                    description: String::from(description),
                }
            },
            // if no regex match, return the default SeqDir initialization
            None => SeqDir {
                path: path.to_path_buf(),
                date: Local::today().naive_local(),
                instrument: String::from(""),
                run: 0,
                position: '?',
                flowcell: String::from(""),
                description: String::from(""),
            }
        }
    }

    /// Path to SeqDir
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// Date of SeqDir creation
    pub fn date(&self) -> Date {
        self.date
    }

    /// Sequencing instrument use to create SeqDir
    pub fn instrument(&self) -> &str {
        &self.instrument
    }

    /// Run number of SeqDir run
    pub fn run(&self) -> u8 {
        self.run
    }

    /// Position of the flowcell in the sequencer
    pub fn position(&self) -> char {
        self.position
    }

    /// Flowcell ID that generated the SeqDir
    pub fn flowcell(&self) -> &str {
        &self.flowcell
    }

    /// General description of the sequencing run
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Create the reserved files, if they are missing from the SeqDir
    pub fn create_reserved_files(&self, verbose: bool, dryrun: bool) {
        // create non-existent reserved files
        if verbose {
            println!("Creating files...");
        }
        for f in &RESERVED_FILENAMES {
            let p = self.path().join(Path::new(&f));
            if !p.as_path().exists() {
                if verbose {
                    println!("  {}", f);
                }
                if !dryrun {
                    create_reserved_file(self, f);
                }
            }
        }
    }

    /// Create the reserved directories, if they are missing from the SeqDir
    pub fn create_reserved_dirs(&self, verbose: bool, dryrun: bool) {
        // create non-existent reserved directories
        if verbose {
            println!("Creating directories...");
        }
        for d in &RESERVED_DIRNAMES {
            let p = self.path().join(Path::new(&d));
            if !p.as_path().exists() {
                if verbose {
                    println!("  {}", d);
                }
                if !dryrun {
                    create_reserved_dir(p);
                }
            }
        }
    }

    /// Relocate HTS files into the appropriate reserved directories
    pub fn relocate_hts_files(&self, verbose: bool, dryrun: bool) {
        // find and relocate FASTQs, if necessary
        if verbose {
            println!("Moving sequencing files...");
        }


        // walk over all HTS files in the folder
        for hts in WalkDir::new(self.path())
            .into_iter()
            .filter_map(|e| e.ok())                             // only consider correct entries
            .filter(|e| e.path().is_file())                     // only consider files
            .filter(|e| detect_filetype(e.path()).is_some())    // only consider HtsFiles
            .map(|e| HtsFile::new(e.path()))                    // convert to HtsFile object
        {
            let destdir: PathBuf;
            // find out where the file needs to go
            match hts.filetype() {
                Hts::FASTX(_) => {
                    destdir = self.path().join(Path::new("FASTQs"));
                }
                Hts::BAM | Hts::SAM | Hts::CRAM => {
                    destdir = self.path().join(Path::new("Aligned"));
                }
                Hts::BCF | Hts::VCF | Hts::MAF => {
                    destdir = self.path().join(Path::new("Variants"));
                },
                Hts::Peak(_) => {
                    destdir = self.path().join(Path::new("Peaks"));
                },
                _ => {
                    destdir = self.path().join(Path::new("Reports"));
                }
            }
            if !dryrun {
                mv_to_dir(hts.path(), destdir.as_path());
            }
            if verbose {
                println!(
                    "  {} -> {}",
                    hts.path().display(),
                    destdir.as_path().join(hts.path().file_name().unwrap()).display()
                );
            }
        }
    }
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
        "README.md"    => create_readme(seq),
        "cluster.yaml" => create_cluster_yaml(seq),
        "Snakefile"    => create_snakefile(seq),
        // exclude config.tsv, make that file separately when you reorganize the FASTQs
        _ => return,
    }
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
        CONFIG = CONFIG.loc[CONFIG.Include == 'No', :]

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
                    path.join(REPORT_DIR, '{sample}_L00{lane}_R{read}_fastqc.zip'),
                    sample=SAMPLES,
                    lane=LANES,
                    read=READS
                ),

        rule rulegraph:
            output:
                'rulegraph.png',
            shell:
                'snakemake --rulegraph | dot -Tpng > {output}'

        # =============================================================================
        # Rules
        # =============================================================================
        # Summaries
        # -----------------------------------------------------------------------------
        rule fastqc:
            input:
                path.join(FASTQ_DIR, '{file}.fastq.gz')
            output:
                path.join(REPORT_DIR, '{file}_fastqc.html'),
                path.join(REPORT_DIR, '{file}_fastqc.zip')
            params:
                '-o {}'.format(REPORT_DIR)
            shell:
                'fastqc {params} {input}'
        rule multiqc:
            input:
                samples = expand(
                    path.join(REPORT_DIR, '{sample}_fastqc.zip'),
                    sample=SAMPLES
                )
            output:
                path.join(REPORT_DIR, 'multiqc_report.html')
            shell:
                'multiqc -f -o {REPORT_DIR} {REPORT_DIR}'

        # Miscellaneous
        # -----------------------------------------------------------------------------
        rule sort_bam_name:
            input:
                '{file}.bam'
            output:
                '{file}.name-sorted.bam',
            shell:
                'sambamba sort -t 8 --tmpdir . -n -p -o {output} {input}'

        rule sort_bam:
            input:
                '{file}.bam'
            output:
                bam = '{file}.sorted.bam',
                idx = '{file}.sorted.bam.bai'
            shell:
                'sambamba sort -t 8 --tmpdir . -p {input}'
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

fn create_config(sd: &SeqDir, dryrun: bool) {
    // return if the config already exists
    if sd.path().join(Path::new("config.tsv")).exists() {
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
    
    // walk over all HTS files in the folder
    for hts in WalkDir::new(sd.path())
        .into_iter()
        .filter_map(|e| e.ok())                             // only consider correct entries
        .filter(|e| e.path().is_file())                     // only consider files
        .filter(|e| detect_filetype(e.path()).is_some())    // only consider HtsFiles
        .map(|e| HtsFile::new(e.path()))                    // convert to HtsFile object
    {
        // don't move directories, only assess FASTQs
        match hts.filetype() {
            Hts::FASTX(Fastx::FASTQ) => {
                let fname = hts.path().file_name().unwrap().to_str().unwrap();
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
    if !dryrun {
        let p = sd.path().join(Path::new("config.tsv"));
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
}

fn mv_to_dir(file: &Path, dir: &Path) {
    rename(file, dir.join(file.file_name().unwrap())).expect("Failed to move file.");
}


/// Organize a directory containing HTS data
pub fn organize(indir: &Path, dryrun: bool, verbose: bool) {
    let sd = SeqDir::new(indir);
    sd.create_reserved_files(verbose, dryrun);
    sd.create_reserved_dirs(verbose, dryrun);
    sd.relocate_hts_files(verbose, dryrun);
    
    // extract sample information from FASTQs, reorganize
    if verbose {
        println!("Extracting sample information...");
    }
    create_config(&sd, dryrun);
    if verbose {
        println!("Done.");
    }
}
