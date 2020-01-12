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
    .subcommand(SubCommand::with_name("info")
        .about("Display metadata about an HTS file")
    )
    .subcommand(SubCommand::with_name("filter-qname")
        .about("Filter an HTS file by its query names")
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
