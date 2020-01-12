pub fn detect_filetype(path: &str) -> String
{
    let mut p = path.to_lowercase();
    let mut zipped = false;
    let ext: &str;
    if p.ends_with(".gz") {
        zipped = true;
        for _n in 0..3 {
            p.pop();
        }
    } else if p.ends_with(".bz2") {
        zipped = true;
        for _n in 0..4 {
            p.pop();
        }
    };

    if p.ends_with(".sam") {
        ext = "SAM";
    } else if p.ends_with(".bam") {
        ext = "SAM";
        zipped = true;
    } else if p.ends_with(".cram") {
        ext = "CRAM";
    } else if p.ends_with(".fasta") {
        ext = "FASTA";
    } else if p.ends_with(".fastq") {
        ext = "FASTQ";
    } else if p.ends_with(".vcf") {
        ext = "VCF";
    } else if p.ends_with(".bcf") {
        ext = "VCF";
        zipped = true;
    } else if p.ends_with(".maf") {
        ext = "MAF";
    } else if p.ends_with(".tbx") {
        ext = "TABIX";
    } else if p.ends_with(".gtf") {
        ext = "TABIX";
    } else if p.ends_with(".gff") {
        ext = "TABIX";
    } else if p.ends_with(".bed") {
        ext = "BED";
    } else if p.ends_with(".bedpe") {
        ext = "BEDPE";
    } else {
        ext = "Unrecognized";
    }

    if zipped {
        return format!("Compressed {}", ext);
    } else {
        return format!("{}", ext);
    }
}