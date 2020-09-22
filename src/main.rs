use clap::{value_t, values_t, App, Arg, SubCommand};
use std::fs::File;
use std::path::Path;

mod data;
mod fastx;
mod align;
mod interval;
mod utils;

use data::organize;
use utils::HtsFile;

fn main() {
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("jaccard")
                .about("Calculate the Jaccard index for each pair in a set of BED files")
                .arg(
                    Arg::with_name("bed")
                        .help("BED file(s) to compare")
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("names")
                        .short("n")
                        .long("names")
                        .help("comma-separated labels for the BED files")
                        .required(false)
                        .default_value(""),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("Output file")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("Extract and print metadata about an HTS file")
                .arg(
                    Arg::with_name("hts")
                        .help("HTS file to extract metadata from")
                        .required(true),
                ),
        )
        // .subcommand(
        //     SubCommand::with_name("filter")
        //         .about("Filter an HTS file by its query names")
        //         .arg(
        //             Arg::with_name("hts")
        //                 .help("A name-sorted HTS file to filter")
        //                 .required(true),
        //         )
        //         .arg(
        //             Arg::with_name("ids")
        //                 .help("Text file containing query name IDs to be removed")
        //                 .required(true),
        //         )
        //         .arg(
        //             Arg::with_name("keep")
        //                 .short("k")
        //                 .long("keep")
        //                 .help("Keep only these IDs, instead of the default which removes them")
        //                 .required(false),
        //         )
        //         .arg(Arg::with_name("output").help("Output file. If not given, print to STDOUT")),
        // )
        .subcommand(
            SubCommand::with_name("org")
                .about("Organize a batch of raw sequencing data")
                .arg(
                    Arg::with_name("dir")
                        .help("Directory to organize")
                        .required(true),
                )
                .arg(
                    Arg::with_name("dryrun")
                        .short("n")
                        .long("dryrun")
                        .takes_value(false)
                        .help("Only show what steps are going to be performed"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .takes_value(false)
                        .help("Show verbose output"),
                ),
        )
        .get_matches();

    if let Some(_o) = _matches.subcommand_matches("info") {
        let hts = value_t!(_o.value_of("hts"), String).unwrap_or_else(|e| e.exit());
        let hts_path = Path::new(&hts);

        // create HtsFile object for the file provided
        let hts_file = HtsFile::new(&hts_path);

        hts_file.print_info();
    } else if let Some(_o) = _matches.subcommand_matches("jaccard") {
        let beds = values_t!(_o.values_of("bed"), String).unwrap_or_else(|e| e.exit());
        let hts_files: Vec<HtsFile> = beds.iter().map(|b| HtsFile::new(Path::new(b))).collect();

        match hts_files.len() {
            1 => println!("Only 1 interval file, which is obviously self-similar."),
            2 => {
                let (i, u, j) = interval::jaccard(&hts_files[0], &hts_files[1]);
                println!("{}, {}, {}", i, u, j);
            }
            _ => {
                // convert the Vec<HtsFile> to Vec<&HtsFile> before calculating
                // Jaccard index on each pair of elements
                let m = interval::multijaccard(&hts_files.iter().collect::<Vec<&HtsFile>>());
                // write to output or print to STDOUT
                if _o.is_present("output") {
                    // get output file as string
                    let outfile =
                        value_t!(_o.value_of("output"), String).unwrap_or_else(|e| e.exit());
                    // create file handle for output
                    let out = File::create(outfile).unwrap();
                    // save to CSV file
                    m.to_csv(out).expect("Unable to save to output.");
                } else {
                    // print to STDOUT
                    m.printstd();
                }
            }
        }
    } else if let Some(_o) = _matches.subcommand_matches("org") {
        let dir = value_t!(_o.value_of("dir"), String).unwrap_or_else(|e| e.exit());
        // let _seqtype = value_t!(_o.value_of("type"), String).unwrap_or_else(|e| e.exit());
        let indir = Path::new(&dir);
        let dryrun = _o.is_present("dryrun");
        let mut verbose = _o.is_present("verbose");
        // if dryrun is flagged, set verbose automatically
        verbose = verbose || dryrun;
        if !indir.exists() {
            println!("{} does not exist. Exiting.", indir.display());
        }
        if !indir.is_dir() {
            println!("{} is not a directory. Exiting.", indir.display());
        } else {
            organize(indir, dryrun, verbose);
        }
    }
}
