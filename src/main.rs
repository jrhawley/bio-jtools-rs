use clap::{App, Arg, crate_version, SubCommand, values_t};

macro_rules! crate_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    }
}

fn main() {
   let matches = App::new("bio-jtools")
    .version(crate_version!())
    .about(crate_description!())
    .subcommand(SubCommand::with_name("multi-jaccard")
        .about("Calculate the Jaccard index for multiple BED files")
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
            .help("HTS file(s) to extract metadata from")
            .multiple(true)
            .required(true)
        )
    )
    .subcommand(SubCommand::with_name("filter-qname")
        .about("Filter a name-sorted HTS file by its query names")
    )
    .subcommand(SubCommand::with_name("org")
        .about("Organize a batch of raw sequencing data")
    )
    .subcommand(SubCommand::with_name("kspec")
        .about("Calculate the k-mer spectra of an HTS file")
    )
    .get_matches();

    println!("Hello, world!");
}
