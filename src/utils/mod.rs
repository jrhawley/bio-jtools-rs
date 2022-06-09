//! # Utilities and helper functions
//!
//! Various helper functions used throughout the `bio-jtools` crate

use crate::align;
use crate::fastx;
use bam::{BamReader, BamWriter, SamReader, SamWriter};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq)]
pub enum Hts {
    Align(Align),
    Fastx(Fastx),
    Variant(Variant),
    Tabix(Tabix),
    Bed(Bed),
    Peak(Peak),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Fastx {
    Fasta,
    Fastq,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Align {
    Bam,
    Cram,
    Sam,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tabix {
    Tab,
    Gff,
    Gtf,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Bed {
    Bed,
    BedPE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Peak {
    BroadPeak,
    GappedPeak,
    NarrowPeak,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Bcf,
    Maf,
    Vcf,
}

const SUPPORTED_EXTENSIONS: [&'static str; 18] = [
    "bam",
    "sam",
    "cram",
    "fasta",
    "fa",
    "fastq",
    "fq",
    "bcf",
    "vcf",
    "maf",
    "tbx",
    "gff",
    "gtf",
    "bed",
    "bedpe",
    "narrowPeak",
    "broadPeak",
    "gappedPeak",
];

/// The structure to manage metadata for an HTS file
pub struct HtsFile {
    path: PathBuf,
    hts_type: Hts,
}

impl HtsFile {
    /// Create new HTS file
    pub fn new(path: &Path) -> HtsFile {
        // check for path existing and that it is a file
        if !path.exists() {
            panic!("File provided does not exist. Please provide a path that exists.");
        } else if !path.is_file() {
            panic!("Path provided is not a file. Please provide the path to a file.");
        }

        // create HtsFile everything looks good so far
        if let Some(hts_type) = detect_filetype(path) {
            HtsFile {
                path: path.to_path_buf(),
                hts_type: hts_type,
            }
        } else {
            panic!(
                "{}",
                format!("Could not parse HTS file type from path. Supported file extensions are (excluding compression): {:?}", SUPPORTED_EXTENSIONS)
            );
        }
    }
    /// HTS file path
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    /// HTS file type
    pub fn filetype(&self) -> Hts {
        self.hts_type
    }

    // /// Print HTS file information
    // pub fn print_info(&self, count_lengths: bool) {
    //     match self.hts_type {
    //         Hts::Fastx(_) => fastx::info(&self, count_lengths),
    //         Hts::Align(Align::Bam) => {
    //             let mut reader = BamReader::from_path(self.path(), 3).unwrap();
    //             align::info(&mut reader, count_lengths)
    //         }
    //         Hts::Align(Align::Sam) => {
    //             let mut reader = SamReader::from_path(self.path()).unwrap();
    //             align::info(&mut reader, count_lengths)
    //         }
    //         _ => unimplemented!(),
    //     }
    // }

    /// Filter reads in an HTS file by their qname.
    pub fn filter(&self, ids: &Path, out: &Path, keep: bool) {
        // match on the combination of input/output files
        match (self.filetype(), detect_filetype(out)) {
            // BAM => BAM
            (Hts::Align(Align::Bam), Some(Hts::Align(Align::Bam))) => {
                let mut reader = BamReader::from_path(self.path(), 3).unwrap();
                let mut writer = BamWriter::from_path(out, reader.header().clone()).unwrap();
                align::filter(&mut reader, ids, &mut writer, keep)
            }
            // BAM => SAM
            (Hts::Align(Align::Bam), Some(Hts::Align(Align::Sam))) => {
                let mut reader = BamReader::from_path(self.path(), 3).unwrap();
                let mut writer = SamWriter::from_path(out, reader.header().clone()).unwrap();
                align::filter(&mut reader, ids, &mut writer, keep)
            }
            // SAM => BAM
            (Hts::Align(Align::Sam), Some(Hts::Align(Align::Bam))) => {
                let mut reader = SamReader::from_path(self.path()).unwrap();
                let mut writer = BamWriter::from_path(out, reader.header().clone()).unwrap();
                align::filter(&mut reader, ids, &mut writer, keep)
            }
            // SAM => SAM
            (Hts::Align(Align::Sam), Some(Hts::Align(Align::Sam))) => {
                let mut reader = SamReader::from_path(self.path()).unwrap();
                let mut writer = SamWriter::from_path(out, reader.header().clone()).unwrap();
                align::filter(&mut reader, ids, &mut writer, keep)
            }
            (Hts::Fastx(_), Some(Hts::Fastx(_))) => fastx::filter(self, ids, out, keep),
            _ => unimplemented!(),
        }
    }
}

/// Determine if a file is compressed or not
fn file_is_zipped(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("gz") | Some("bz2") => true,
            _ => false,
        },
        None => false,
    }
}

pub fn detect_filetype(path: &Path) -> Option<Hts> {
    let stem: &Path;
    // strip zipped extension if it's a zipped file
    if file_is_zipped(path) {
        stem = Path::new(path.file_stem().unwrap());
    } else {
        stem = path;
    }

    match stem.extension() {
        Some(ext) => match ext.to_str() {
            Some("bam") => Some(Hts::Align(Align::Bam)),
            Some("sam") => Some(Hts::Align(Align::Sam)),
            Some("cram") => Some(Hts::Align(Align::Cram)),
            Some("fasta") | Some("fa") => Some(Hts::Fastx(Fastx::Fasta)),
            Some("fastq") | Some("fq") => Some(Hts::Fastx(Fastx::Fastq)),
            Some("bcf") => Some(Hts::Variant(Variant::Bcf)),
            Some("vcf") => Some(Hts::Variant(Variant::Vcf)),
            Some("maf") => Some(Hts::Variant(Variant::Maf)),
            Some("tbx") => Some(Hts::Tabix(Tabix::Tab)),
            Some("gff") => Some(Hts::Tabix(Tabix::Gff)),
            Some("gtf") => Some(Hts::Tabix(Tabix::Gtf)),
            Some("bed") => Some(Hts::Bed(Bed::Bed)),
            Some("bedpe") => Some(Hts::Bed(Bed::BedPE)),
            Some("narrowPeak") => Some(Hts::Peak(Peak::NarrowPeak)),
            Some("broadPeak") => Some(Hts::Peak(Peak::BroadPeak)),
            Some("gappedPeak") => Some(Hts::Peak(Peak::GappedPeak)),
            _ => None,
        },
        None => None,
    }
}
