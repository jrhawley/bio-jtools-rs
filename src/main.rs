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
        .arg(Arg::with_name("word")
            .help("Password(s) listend on the terminal")
            .multiple(true)
            .required(true)
        )
    )
    .subcommand(SubCommand::with_name("fqinfo")
        .about("Display metadata about a FASTQ file")
    )
    .subcommand(SubCommand::with_name("filter-qname")
        .about("Filter a SAM/BAM file by its query names")
    )
    .subcommand(SubCommand::with_name("org")
        .about("Organize a batch of raw sequencing data")
    )
    .get_matches();

    println!("Hello, world!");
}
