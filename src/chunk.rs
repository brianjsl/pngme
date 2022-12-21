use std::{fmt, str, error};
use crc::{Crc, Algorithm, CRC_32_ISO_HDLC};
use crate::Result;

use crate::chunk_type;

pub const ISO_3309: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub struct Chunk {
    length: u32,
    chunk_type: chunk_type::ChunkType, 
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn is_valid_crc(chunk_type: &chunk_type::ChunkType, data: &Vec<u8>, crc: u32) -> bool{
        let mut x: Vec<u8> = chunk_type.bytes().to_vec();
        x.extend(data);

        let crc_output = ISO_3309.checksum(&x);

        crc_output == crc 
    }

    pub fn new(chunk_type: chunk_type::ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;
        let mut x: Vec<u8> = chunk_type.bytes().to_vec();
        x.extend(&data);
        
        let crc = ISO_3309.checksum(&x);

        Self {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &chunk_type::ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let bytes: &[u8] = &self.data;
        match str::from_utf8(bytes) {
            Ok(s) => Ok(String::from(s)),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = self.length.to_be_bytes();
        let chunk_type_bytes = self.chunk_type.bytes();
        let crc_bytes = self.crc.to_be_bytes().to_vec();

        let bytes: Vec<u8> = length_bytes.iter()
                            .chain(chunk_type_bytes.iter())
                            .chain(self.data.iter())
                            .chain(crc_bytes.iter())
                            .copied().collect();
        bytes
    }

}

impl TryFrom<&[u8]> for Chunk {
    
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self> {

        //get length
        let length_bytes: [u8;4] = value[..4].try_into()?;
        let length: u32 = u32::from_be_bytes(length_bytes);

        //get chunk_type
        let chunk_type_bytes: [u8;4] = value[4..8].try_into()?;
        let chunk_type = chunk_type::ChunkType::try_from(chunk_type_bytes)?;

        //get data
        let data: Vec<u8>= value[8..(8+length as usize)].try_into()?;

        //get crc
        let crc_bytes: [u8;4] = value[(8 + length as usize)..].try_into()?;
        let crc = u32::from_be_bytes(crc_bytes);

        match Self::is_valid_crc(&chunk_type, &data, crc) {
            true => {}, 
            false => {return Err(Box::new(InvalidCrcError))}
        }

        Ok(Self {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc
        })
    }

}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_str = self.data_as_string().unwrap(); 
        write!(f, "{}", data_str)
    }
}

#[derive(Debug)]
struct InvalidCrcError;

impl fmt::Display for InvalidCrcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Crc Bit!")
    }
}

impl error::Error for InvalidCrcError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}