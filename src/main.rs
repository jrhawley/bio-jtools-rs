use clap::{App, Arg, crate_version, SubCommand, value_t};
mod utils;
mod fastx;

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
            .help("comma-separated labels for the BED files")
        )
        .arg(Arg::with_name("prefix")
            .help("Output file prefix. `<prefix>.tsv` and `<prefix>.png` are created")
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
        println!("{}", utils::detect_filetype(&hts));
        fastx::fx_info(&hts);
    }
}
