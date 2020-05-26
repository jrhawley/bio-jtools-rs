use std::fs::rename;
use chrono::DateTime;
use regex::Regex;

struct SeqDir {
    date: &str,
    instrument: &str,
    run: &str,
    position: char,
    flowcell: &str,
    date_submitted: DateTime,
    date_received: DateTime,
    description: &str
}

let reserved_dirnames = vec!["Reports", "FASTQs", "Trimmed", "Aligned", "Peaks", "Contacts"];
let reserved_filenames = vec!["README.md", "cluster.yaml", "Snakefile", "setup.log", "config.tsv"];
let fq_regex = Regex::new(r"^([A-Za-z0-9-_]+)_S([1-9][0-9]?)_L00(\d)_(I[1-3]|R[1-3])_001\.f(ast)?q(\.gz)?$").unwrap();
let dir_regex = Regex::new(r"^([0-9]{2})(0?[1-9]|1[012])(0[1-9]|[12]\d|3[01])_(\w{6})_(\d{4})_(A|B)(\w{9})(.*)?/?$").unwrap();

fn seqdir_from_path (dirname: &str) -> SeqDir
{
    cap = dir_regex.captures_iter(dirname);
    println!("{}{}{}", &cap[1], &cap[2], &cap[3]);
    // let date = DateTime::parse_from_str(&cap[1])
    // let seqdir = SeqDir::new(
    //     date = 
    // )
    return SeqDir::new();
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


pub fn organize(_indir: &str, _outdir: &str, _seqtype: &str)
{
    // record the steps take during setup
    seqdir_from_path(_indir);
}