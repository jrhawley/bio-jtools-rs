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

impl TryFrom<&[u8]> for RecordName {
    type Error = RecordError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value[0..3] == SRA_RNAME_PREFIX[0..3] {
            return Ok(RecordName::SequenceReadArchive);
        } else {
            return Ok(RecordName::CasavaV1_8);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check_read_name_fmt(rname: &str, exp: Result<RecordName, RecordError>) {
        let rname_bytes = rname.as_bytes();
        let obs = RecordName::try_from(rname_bytes);

        assert_eq!(obs, exp);
    }

    #[test]
    fn casavav1_is_casava() {
        let rname = "HWUSI-EAS100R:6:73:941:1973#0/1";

        check_read_name_fmt(rname, Ok(RecordName::CasavaV1_8));
    }

    #[test]
    fn casavav1_4_is_casava() {
        let rname = "HWUSI-EAS100R:6:73:941:1973#ACTAGC/1";

        check_read_name_fmt(rname, Ok(RecordName::CasavaV1_8));
    }

    #[test]
    fn casavav1_8_is_casava() {
        let rname = "EAS139:136:FC706VJ:2:2104:15343:197393 1:Y:18:ATCACG";

        check_read_name_fmt(rname, Ok(RecordName::CasavaV1_8));
    }

    #[test]
    fn casavav1_8_without_sample_index_is_casava() {
        let rname = "EAS139:136:FC706VJ:2:2104:15343:197393 1:Y:18:1";

        check_read_name_fmt(rname, Ok(RecordName::CasavaV1_8));
    }

    #[test]
    fn srr_is_sra() {
        let rname = "SRR001666.1 071112_SLXA-EAS1_s_7:5:1:817:345 length=36";

        check_read_name_fmt(rname, Ok(RecordName::SequenceReadArchive));
    }

    #[test]
    fn srr_without_original_info_is_sra() {
        let rname = "SRR001666.1 length=36";

        check_read_name_fmt(rname, Ok(RecordName::SequenceReadArchive));
    }

    #[test]
    fn srr_without_original_info_nor_length_is_sra() {
        let rname = "SRR001666.1";

        check_read_name_fmt(rname, Ok(RecordName::SequenceReadArchive));
    }

    #[test]
    fn srr_in_origfmt_is_casava() {
        let rname = "071112_SLXA-EAS1_s_7:5:1:817:345";

        check_read_name_fmt(rname, Ok(RecordName::CasavaV1_8));
    }
}
