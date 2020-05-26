use clap::{App, Arg, crate_version, SubCommand, value_t, values_t};
mod utils;
mod fastx;
mod interval;
use std::path::Path;

macro_rules! crate_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    }
}

fn main() {
    let _matches = App::new("bio-jtools")
    .version(crate_version!())
    .about(crate_description!())
    .subcommand(SubCommand::with_name("jaccard")
        .about("Calculate the Jaccard index for each pair in a set of BED files")
        .arg(Arg::with_name("bed")
            .help("BED file(s) to compare")
            .multiple(true)
            .required(true)
        )
        .arg(Arg::with_name("names")
            .short("n")
            .long("names")
            .help("comma-separated labels for the BED files")
            .required(false)
            .default_value("")
        )
        .arg(Arg::with_name("prefix")
            .help("Output file prefix. `<prefix>.tsv` and `<prefix>.png` are created")
            .short("o")
            .long("prefix")
            .required(false)
            .default_value("output")
        )
    )
    .subcommand(SubCommand::with_name("info")
        .about("Extract and print metadata about an HTS file")
        .arg(Arg::with_name("hts")
            .help("HTS file to extract metadata from")
            .required(true)
        )
    )
    .subcommand(SubCommand::with_name("filter")
        .about("Filter an HTS file by its query names")
        .arg(Arg::with_name("hts")
            .help("A name-sorted HTS file to filter")
            .required(true)
        )
        .arg(Arg::with_name("ids")
            .help("Text file containing query name IDs to be removed")
            .required(true)
        )
        .arg(Arg::with_name("keep")
             .short("k")
             .long("keep")
             .help("Keep only these IDs, instead of the default which removes them")
             .required(false)
        )
        .arg(Arg::with_name("output")
            .help("Output file. If not given, print to STDOUT")
        )
    )
    .subcommand(SubCommand::with_name("org")
        .about("Organize a batch of raw sequencing data")
        .arg(Arg::with_name("dir")
            .help("Directory to organize")
            .required(true)
        )
        .arg(Arg::with_name("outdir")
            .help("New path for input directory")
            .required(false)
        )
        .arg(Arg::with_name("type")
            .help("New path for input directory")
            .possible_values(&["mix", "atac", "chip", "bs", "dna", "rna", "hic"])
            .default_value("mix")
        )
    )
    .subcommand(SubCommand::with_name("kspec")
        .about("Calculate the k-mer spectra of an HTS file")
        .arg(Arg::with_name("hts")
            .help("HTS file(s) to calculate spectra from")
            .required(true)
        )
        .arg(Arg::with_name("k")
            .help("Length of k-mer")
            .required(true)
        )
    )
    .get_matches();

    if let Some(o) = _matches.subcommand_matches("info") {
        let hts = value_t!(o.value_of("hts"), String).unwrap_or_else(|e| e.exit());
        let hts_path = Path::new(&hts);
        // check that supplied HTS file exists
        if !hts_path.exists() {
            println!("{} does not exist. Exiting.", &hts);
            return;
        }
        let ftype = utils::detect_filetype(&hts_path);

        match ftype {
            "FASTQ" => fastx::info(hts_path),
            _ => unimplemented!(),
        }

        // run info on supplied FASTQ
        
    } else if let Some(o) = _matches.subcommand_matches("jaccard") {
        let beds = values_t!(o.values_of("bed"), String).unwrap_or_else(|e| e.exit());
        // check that supplied BED files exists
        for b in &beds {
            if !Path::new(&b).exists() {
                println!("{} does not exist. Exiting.", &b);
                return;
            }
        }
        if beds.len() == 2 {
            interval::jaccard_path(&beds[0], &beds[1]);
        }
    } else if let Some(o) = _matches.subcommand_matches("org") {
        unimplemented!();
    } else if let Some(o) = _matches.subcommand_matches("kspec") {
        unimplemented!();
    } else if let Some(o) = _matches.subcommand_matches("filter") {
        unimplemented!();
    }
}
