//! Header formats for FASTA and FASTQ files.

use std::borrow::Cow;

use super::error::RecordError;

/// Separator ASCII value in read names for metadata elements
///
/// In read names, pieces of information in the byte string are split by ":".
pub const ILLUMINA_SEPARATOR_ASCII_CODE: u8 = 58;

/// Separator ASCII value in read names for groups of information
///
/// In read names, the byte string is split by " ".
pub const RNAME_SEPARATOR_ASCII_CODE: u8 = 32;

/// Few few bytes in a read name coming from the Sequence Read Archive
///
/// This is "SRR" encoded in ASCII bytes.
const SRA_RNAME_PREFIX: &[u8] = "SRR".as_bytes();

#[derive(Debug, PartialEq)]
pub enum RecordName {
    CasavaV1_8,
    SequenceReadArchive,
}

/// FASTQ ID from Casava-processed files, version >=1.8
#[derive(Debug)]
pub(crate) struct CasavaV1_8Name {
    /// Instrument name
    instrument: Option<String>,

    /// Flow cell lane
    lane: Option<u8>,

    /// Tile number within the flow cell lane
    tile: Option<u8>,

    /// x-coordinate of the cluster within the tile
    x: Option<u8>,

    /// y-coordinate of the cluster within the tile
    y: Option<u8>,

    /// Index number for a multi-plexed sample
    /// (0 for no indexing)
    sample_index: Option<u8>,

    /// Member of a pair
    pair_member: Option<u8>,
}

/// FASTQ ID from FASTQ files processes by the Sequence Read Archive
#[derive(Debug)]
pub(crate) struct SraName<'id> {
    /// Clobbered record ID
    id: Cow<'id, [u8]>,

    /// Short description or other info
    description: Cow<'id, [u8]>,

    /// Length of the record
    length: Cow<'id, [u8]>,
}
