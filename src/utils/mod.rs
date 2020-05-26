use std::path::Path;

pub fn file_is_zipped(path: &Path) -> bool {
    let ext = path.extension().unwrap().to_str().unwrap();
    match ext {
        "gz" | "bz2" => true,
        _ => false,
    }
}

pub fn detect_filetype(path: &Path) -> &str {
    let stem: &Path;
    if file_is_zipped(path) {
        stem = Path::new(path.file_stem().unwrap());
    } else {
        stem = path;
    }

    match stem.extension().unwrap().to_str().unwrap() {
        "bam" | "sam" => "SAM",
        "cram" => "CRAM",
        "fasta" => "FASTA",
        "fastq" => "FASTQ",
        "vcf" | "bcf" => "VCF",
        "maf" => "MAF",
        "tbx" | "gff" | "gtf" => "TABIX",
        "bed" => "BED",
        "bedpe" => "BEDPE",
        _ => "Unrecognized",
    }
}
