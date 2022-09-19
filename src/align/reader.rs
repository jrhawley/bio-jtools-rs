//! Unified interface for reading SAM, BAM, and CRAM files.

use bam::{BamReader, SamReader};
use std::io::{BufRead, Read};

pub enum SamBamCramReader<R1: BufRead, R2: Read> {
    Sam(SamReader<R1>),
    Bam(BamReader<R2>),
    Cram,
}
