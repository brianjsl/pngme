//!
//! 4 byte PNG Chunk Type (TypeCode) Object 
//!
use std::{fmt, str, convert::TryFrom, error};


/// 4 byte ChunkType Field of Chunk Object 
#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {

    fn is_valid_b(val: u8) -> bool {
        (65<= val && val <= 90) || (97 <= val && val <= 122) 
    }

    /// Gets the fifth bit of the val as a bool
    fn get_fifth_bit(val: u8) -> bool {
        (val >> 5 & 1) != 0
    }

    /// Checks whether a ChunkType has valid bytes
    fn has_valid_bytes(val: [u8;4]) -> bool {
        ChunkType::is_valid_b(val[0]) && ChunkType::is_valid_b(val[1]) 
        && ChunkType::is_valid_b(val[2]) && ChunkType::is_valid_b(val[3]) 
    }

    /// Returns a byte representation of the ChunkType
    pub fn bytes(&self) -> [u8;4] {
        [self.0, self.1, self.2, self.3]
    }

    /// Checks if a ChunkType is valid
    pub fn is_valid(&self) -> bool {
        Self::has_valid_bytes(self.bytes()) && self.is_reserved_bit_valid()
    }

    /// Checks if a ChunkType is critical or ancilliary
    pub fn is_critical(&self) -> bool {
        !ChunkType::get_fifth_bit(self.0) 
    }

    /// Checks if a ChunkType is public or not
    pub fn is_public(&self) -> bool {
        !ChunkType::get_fifth_bit(self.1)
    }

    /// Checks if the Reserved bit of the ChunkType is valid.
    pub fn is_reserved_bit_valid(&self) -> bool {
        !ChunkType::get_fifth_bit(self.2) 
    }

    /// Checks if the Chunk is safe to copy.
    pub fn is_safe_to_copy(&self) -> bool {
        ChunkType::get_fifth_bit(self.3)
    }
}

/// Converts a byte array to a ChunkType object
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    //check that the bytes of the chunktype are valid
    fn try_from(value: [u8;4]) -> crate::Result<Self>{
        match ChunkType::has_valid_bytes(value) {
            true => Ok(ChunkType(value[0], value[1], value[2], value[3])), 
            false => Err(Box::new(InvalidChunkTypeError))
        }
    }
}

/// Converts a string to a ChunkType object
impl str::FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let bytes: &[u8] = s.as_bytes();
        assert_eq!(bytes.len(), 4);
        let bytes_owned: [u8;4] = [bytes[0], bytes[1], bytes[2], bytes[3]];

        ChunkType::try_from(bytes_owned)
    }
}

/// Displays the ChunkType object as a string
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = &self.bytes();
        let ct_as_str = str::from_utf8(bytes).unwrap();
        write!(f, "{}", ct_as_str)
    }
}

/// Default ChunkType Creation Error
#[derive(Debug)]
struct InvalidChunkTypeError;

impl fmt::Display for InvalidChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Chunk!")
    }
}

impl error::Error for InvalidChunkTypeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        println!("{}", chunk.is_err());
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}