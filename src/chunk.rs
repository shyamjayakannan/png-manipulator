#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;

use crate::{chunk_type::ChunkType, Error, Result};

use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self {
            length: data.len() as u32,
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
                .checksum(&[&chunk_type.bytes(), &data[..]].concat()),
            chunk_type,
            chunk_data: data,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.length.to_be_bytes().as_ref(),
            &self.chunk_type.bytes(),
            &self.chunk_data,
            self.crc.to_be_bytes().as_ref(),
        ]
        .concat()
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let chunk_type =
            ChunkType::try_from(<&[u8] as TryInto<[u8; 4]>>::try_into(&value[4..8]).unwrap())
                .unwrap();
        let chunk_data = value[8..value.len() - 4].to_vec();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC)
            .checksum(&[&chunk_type.bytes(), &chunk_data[..]].concat());

        if crc == u32::from_be_bytes(value[value.len() - 4..].try_into().unwrap()) {
            Ok(Self {
                length: u32::from_be_bytes(value[..4].try_into().unwrap()),
                chunk_type,
                chunk_data,
                crc,
            })
        } else {
            Err("crc incorrect")?
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::chunk_type::ChunkType;
//     use std::str::FromStr;

//     fn testing_chunk() -> Chunk {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         Chunk::try_from(chunk_data.as_ref()).unwrap()
//     }

//     #[test]
//     fn test_new_chunk() {
//         let chunk_type = ChunkType::from_str("RuSt").unwrap();
//         let data = "This is where your secret message will be!"
//             .as_bytes()
//             .to_vec();
//         let chunk = Chunk::new(chunk_type, data);
//         assert_eq!(chunk.length(), 42);
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_chunk_length() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.length(), 42);
//     }

//     #[test]
//     fn test_chunk_type() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//     }

//     #[test]
//     fn test_chunk_string() {
//         let chunk = testing_chunk();
//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");
//         assert_eq!(chunk_string, expected_chunk_string);
//     }

//     #[test]
//     fn test_chunk_crc() {
//         let chunk = testing_chunk();
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_valid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

//         let chunk_string = chunk.data_as_string().unwrap();
//         let expected_chunk_string = String::from("This is where your secret message will be!");

//         assert_eq!(chunk.length(), 42);
//         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
//         assert_eq!(chunk_string, expected_chunk_string);
//         assert_eq!(chunk.crc(), 2882656334);
//     }

//     #[test]
//     fn test_invalid_chunk_from_bytes() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656333;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk = Chunk::try_from(chunk_data.as_ref());

//         assert!(chunk.is_err());
//     }

//     #[test]
//     pub fn test_chunk_trait_impls() {
//         let data_length: u32 = 42;
//         let chunk_type = "RuSt".as_bytes();
//         let message_bytes = "This is where your secret message will be!".as_bytes();
//         let crc: u32 = 2882656334;

//         let chunk_data: Vec<u8> = data_length
//             .to_be_bytes()
//             .iter()
//             .chain(chunk_type.iter())
//             .chain(message_bytes.iter())
//             .chain(crc.to_be_bytes().iter())
//             .copied()
//             .collect();

//         let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

//         let _chunk_string = format!("{}", chunk);
//     }
// }
