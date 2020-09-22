use std::path::{Path, PathBuf};
use crate::fastx;

#[derive(Clone, Copy, PartialEq)]
pub enum Hts {
    SAM,
    CRAM,
    FASTX(Fastx),
    VCF,
    MAF,
    TABIX(Tabix),
    BED(Bed),
    Peak(Peak),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Fastx {
    FASTA,
    FASTQ,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tabix {
    Tab,
    GFF,
    GTF,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Bed {
    BED,
    BEDPE,
    Peak,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Peak {
    BroadPeak,
    GappedPeak,
    NarrowPeak,
}

/// The structure to manage metadata for an HTS file
pub struct HtsFile {
    path: PathBuf,
    hts_type: Hts,
    zipped: bool,
}

impl HtsFile {
    /// Create new HTS file
    pub fn new(path: &Path) -> HtsFile {
        if let Some(hts_type) = detect_filetype(path) {
            HtsFile {
                path: path.to_path_buf(),
                hts_type: hts_type,
                zipped: file_is_zipped(path),
            }
        } else {
            panic!("Error creating new HtsFile");
        }
    }
    /// Determine if the HTS file is compressed or not
    pub fn is_zipped(&self) -> bool {
        self.zipped
    }
    /// HTS file path
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    /// HTS file type
    pub fn filetype(&self) -> Hts {
        self.hts_type
    }

    /// Print HTS file information
    pub fn print_info(&self) {
        match self.hts_type {
            Hts::FASTX(_) => fastx::info(&self.path()),
            _ => unimplemented!(),
        }
    }
}

/// Determine if a file is compressed or not
fn file_is_zipped(path: &Path) -> bool {
    let ext = path.extension().unwrap().to_str().unwrap();
    match ext {
        "gz" | "bz2" => true,
        _ => false,
    }
}

fn detect_filetype(path: &Path) -> Option<Hts> {
    if !path.exists() {
        return None;
    }
    let stem: &Path;
    if file_is_zipped(path) {
        stem = Path::new(path.file_stem().unwrap());
    } else {
        stem = path;
    }

    match stem.extension().unwrap().to_str().unwrap() {
        "bam" | "sam" => Some(Hts::SAM),
        "cram"        => Some(Hts::CRAM),
        "fasta"       => Some(Hts::FASTX(Fastx::FASTA)),
        "fastq"       => Some(Hts::FASTX(Fastx::FASTQ)),
        "vcf" | "bcf" => Some(Hts::VCF),
        "maf"         => Some(Hts::MAF),
        "tbx"         => Some(Hts::TABIX(Tabix::Tab)),
        "gff"         => Some(Hts::TABIX(Tabix::GFF)),
        "gtf"         => Some(Hts::TABIX(Tabix::GTF)),
        "bed"         => Some(Hts::BED(Bed::BED)),
        "bedpe"       => Some(Hts::BED(Bed::BEDPE)),
        _ => None,
    }
}
