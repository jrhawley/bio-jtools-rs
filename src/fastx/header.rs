//! Header formats for FASTX files.

use std::borrow::Cow;

pub(crate) enum FastqId {
    CasavaV1(CasavaV1Id),
    CasavaV1_8(CasavaV1_8Id),
    SequenceReadArchive(SraId),
}

/// FASTQ ID from Casava-processed files, version <1.4
#[derive(Debug)]
pub(crate) struct CasavaV1Id {
    /// Instrument name
    instrument: String,

    /// Flow cell lane
    lane: u8,

    /// Tile number within the flow cell lane
    tile: u8,

    /// x-coordinate of the cluster within the tile
    x: u8,

    /// y-coordinate of the cluster within the tile
    y: u8,

    /// Index number for a multi-plexed sample
    /// (0 for no indexing)
    sample_index: u8,

    /// Member of a pair
    pair_member: u8,
}

/// FASTQ ID from Casava-processed files, version >=1.4, <1.8
#[derive(Debug)]
pub(crate) struct CasavaV1_4Id {
    /// Instrument name
    instrument: String,

    /// Flow cell lane
    lane: u8,

    /// Tile number within the flow cell lane
    tile: u8,

    /// x-coordinate of the cluster within the tile
    x: u8,

    /// y-coordinate of the cluster within the tile
    y: u8,

    /// Index number for a multi-plexed sample
    /// (0 for no indexing)
    sample_index: u8,

    /// Member of a pair
    pair_member: u8,
}

/// FASTQ ID from Casava-processed files, version >=1.8
#[derive(Debug)]
pub(crate) struct CasavaV1_8Id {
    /// Instrument name
    instrument: String,

    /// Flow cell lane
    lane: u8,

    /// Tile number within the flow cell lane
    tile: u8,

    /// x-coordinate of the cluster within the tile
    x: u8,

    /// y-coordinate of the cluster within the tile
    y: u8,

    /// Index number for a multi-plexed sample
    /// (0 for no indexing)
    sample_index: u8,

    /// Member of a pair
    pair_member: u8,
}

/// FASTQ ID from FASTQ files processes by the Sequence Read Archive
#[derive(Debug)]
pub(crate) struct SraId {
    // /// Clobbered record ID
    // id: Cow<'id, [u8]>,

    // /// Short description or other info
    // description: Cow<'id, [u8]>,

    // /// Length of the record
    // length: Cow<'id, [u8]>,
}
