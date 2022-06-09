//! # bio-jtools
//!
//! A collection of utilities for handling batches of DNA sequencing files.

mod align;
mod cli;
mod data;
mod fastx;
mod interval;
mod utils;

use clap::Parser;
use cli::Cli;
use std::fs::File;
use std::path::Path;
use data::organize;
use utils::HtsFile;

use crate::cli::CliOpt;

fn main() {
    let args = Cli::parse();

    match args.cmd {
        cli::SubCmd::Info(opts) => {
            // println!("{:#?}", opts);
            opts.exec();
        }
        cli::SubCmd::Filter => {}
        cli::SubCmd::Organize => {}
    }

    // let _matches = App::new(env!("CARGO_PKG_NAME"))
    //     .subcommand(
    //         SubCommand::with_name("jaccard")
    //             .about("Calculate the Jaccard index for each pair in a set of BED files")
    //             .arg(
    //                 Arg::with_name("bed")
    //                     .help("BED file(s) to compare")
    //                     .multiple(true)
    //                     .required(true),
    //             )
    //             .arg(
    //                 Arg::with_name("names")
    //                     .short("n")
    //                     .long("names")
    //                     .help("Comma-separated labels for the BED files")
    //                     .required(false)
    //                     .default_value(""),
    //             )
    //             .arg(
    //                 Arg::with_name("output")
    //                     .short("o")
    //                     .long("output")
    //                     .help("Output file")
    //                     .required(false)
    //                     .takes_value(true),
    //             ),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("filter")
    //             .arg(
    //                 Arg::with_name("hts")
    //                     .help("A name-sorted HTS file to filter")
    //                     .required(true),
    //             )
    //             .arg(
    //                 Arg::with_name("ids")
    //                     .help("Text file containing query name IDs to be removed")
    //                     .required(true),
    //             )
    //             .arg(
    //                 Arg::with_name("keep")
    //                     .short("k")
    //                     .long("keep")
    //                     .help("Keep only these IDs, instead of the default which removes them")
    //                     .default_value("false")
    //                     .required(false),
    //             )
    //             .arg(
    //                 Arg::with_name("output")
    //                     .help("Filtered output file")
    //                     .required(true),
    //             ),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("org")
    //             .arg(
    //                 Arg::with_name("dir")
    //                     .help("Directory to organize")
    //                     .required(true),
    //             )
    //             .arg(
    //                 Arg::with_name("dryrun")
    //                     .short("n")
    //                     .long("dryrun")
    //                     .takes_value(false)
    //                     .help("Only show what steps are going to be performed"),
    //             )
    //             .arg(
    //                 Arg::with_name("verbose")
    //                     .short("v")
    //                     .long("verbose")
    //                     .takes_value(false)
    //                     .help("Show verbose output"),
    //             ),
    //     )
    //     .get_matches();

    // if let Some(_subcmd_args) = _matches.subcommand_matches("info") {
    //     let hts = value_t!(_subcmd_args.value_of("hts"), String).unwrap_or_else(|e| e.exit());
    //     // let count_lengths = true;
    //     let count_lengths = _subcmd_args.is_present("count_lengths");

    //     // create HtsFile object for the file provided
    //     let hts_path = Path::new(&hts);
    //     let hts_file = HtsFile::new(&hts_path);

    //     hts_file.print_info(count_lengths);
    // } else if let Some(_subcmd_args) = _matches.subcommand_matches("jaccard") {
    //     let beds = values_t!(_subcmd_args.values_of("bed"), String).unwrap_or_else(|e| e.exit());
    //     let hts_files: Vec<HtsFile> = beds.iter().map(|b| HtsFile::new(Path::new(b))).collect();

    //     match hts_files.len() {
    //         1 => println!("Only 1 interval file, which is obviously self-similar."),
    //         2 => {
    //             let (i, u, j) = interval::jaccard(&hts_files[0], &hts_files[1]);
    //             println!("{}, {}, {}", i, u, j);
    //         }
    //         _ => {
    //             // convert the Vec<HtsFile> to Vec<&HtsFile> before calculating
    //             // Jaccard index on each pair of elements
    //             let m = interval::multijaccard(&hts_files.iter().collect::<Vec<&HtsFile>>());
    //             // write to output or print to STDOUT
    //             if _subcmd_args.is_present("output") {
    //                 // get output file as string
    //                 let outfile = value_t!(_subcmd_args.value_of("output"), String)
    //                     .unwrap_or_else(|e| e.exit());
    //                 // create file handle for output
    //                 let out = File::create(outfile).unwrap();
    //                 // save to CSV file
    //                 m.to_csv(out).expect("Unable to save to output.");
    //             } else {
    //                 // print to STDOUT
    //                 m.printstd();
    //             }
    //         }
    //     }
    // } else if let Some(_subcmd_args) = _matches.subcommand_matches("org") {
    //     let dir = value_t!(_subcmd_args.value_of("dir"), String).unwrap_or_else(|e| e.exit());
    //     // let _seqtype = value_t!(_subcmd_args.value_of("type"), String).unwrap_or_else(|e| e.exit());
    //     let indir = Path::new(&dir);
    //     let dryrun = _subcmd_args.is_present("dryrun");
    //     let mut verbose = _subcmd_args.is_present("verbose");
    //     // if dryrun is flagged, set verbose automatically
    //     verbose = verbose || dryrun;
    //     if !indir.exists() {
    //         println!("{} does not exist. Exiting.", indir.display());
    //     }
    //     if !indir.is_dir() {
    //         println!("{} is not a directory. Exiting.", indir.display());
    //     } else {
    //         organize(indir, dryrun, verbose);
    //     }
    // } else if let Some(_subcmd_args) = _matches.subcommand_matches("filter") {
    //     let hts = value_t!(_subcmd_args.value_of("hts"), String).unwrap_or_else(|e| e.exit());
    //     let ids = value_t!(_subcmd_args.value_of("ids"), String).unwrap_or_else(|e| e.exit());
    //     let output = value_t!(_subcmd_args.value_of("output"), String).unwrap_or_else(|e| e.exit());
    //     let keep = _subcmd_args.is_present("keep");

    //     let hts_path = Path::new(&hts);
    //     let ids_path = Path::new(&ids);
    //     let out_path = Path::new(&output);

    //     let hts_file = HtsFile::new(&hts_path);
    //     hts_file.filter(ids_path, out_path, keep);
    // }
}
