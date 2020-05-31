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
    let p = seq.path.join(Path::new("cluster.yaml"));
    let mut file = match File::create(&p) {
        // The `description` method of `io::Error` returns a string that
        Err(why) => panic!("couldn't open {}: {}", p.display(), why.to_string()),
        Ok(file) => file,
    };
    let text = b"__default__:
  params: '-p all --export=ALL -t 1-00:00:00 --mem 6G'
";
    match file.write_all(text) {
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
    let text = 
        b"# =============================================================================
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
";
    match file.write_all(text) {
        Err(why) => panic!("couldn't write to {}: {}", p.display(), why.to_string()),
        Ok(_) => return,
    }
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
    let reserved_filenames = vec!["README.md", "Snakefile", "cluster.yaml", "config.tsv"];
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
