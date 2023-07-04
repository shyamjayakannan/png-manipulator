#![allow(unused_variables)]
#![allow(dead_code)]

use std::{str::{FromStr, from_utf8}, fmt};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    fn is_critical(&self) -> bool {
        !(self.0[0] & (1 << 5) != 0)
    }

    fn is_public(&self) -> bool {
        !(self.0[1] & (1 << 5) != 0)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        !(self.0[2] & (1 << 5) != 0)
    }

    fn is_safe_to_copy(&self) -> bool {
        self.0[3] & (1 << 5) != 0
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() {
            if s.len() == 4 {
                let arr: [u8; 4] = s.as_bytes()[..4].try_into().unwrap();
                let arr: Vec<u8> = arr.into_iter().take_while(|a| a.is_ascii_lowercase() || a.is_ascii_uppercase()).collect();

                match arr.try_into() {
                    Ok(arr) => Ok(Self(arr)),
                    Err(_) => Err("length not 4")
                }  
            }
            else {
                Err("length not 4")
            }
        }
        else {
            Err("not all characters are ascii")
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_utf8(&self.0).unwrap())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::convert::TryFrom;
//     use std::str::FromStr;

//     #[test]
//     pub fn test_chunk_type_from_bytes() {
//         let expected = [82, 117, 83, 116];
//         let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

//         assert_eq!(expected, actual.bytes());
//     }

//     #[test]
//     pub fn test_chunk_type_from_str() {
//         let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
//         let actual = ChunkType::from_str("RuSt").unwrap();
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     pub fn test_chunk_type_is_critical() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_critical());
//     }

//     #[test]
//     pub fn test_chunk_type_is_not_critical() {
//         let chunk = ChunkType::from_str("ruSt").unwrap();
//         assert!(!chunk.is_critical());
//     }

//     #[test]
//     pub fn test_chunk_type_is_public() {
//         let chunk = ChunkType::from_str("RUSt").unwrap();
//         assert!(chunk.is_public());
//     }

//     #[test]
//     pub fn test_chunk_type_is_not_public() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(!chunk.is_public());
//     }

//     #[test]
//     pub fn test_chunk_type_is_reserved_bit_valid() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_reserved_bit_valid());
//     }

//     #[test]
//     pub fn test_chunk_type_is_reserved_bit_invalid() {
//         let chunk = ChunkType::from_str("Rust").unwrap();
//         assert!(!chunk.is_reserved_bit_valid());
//     }

//     #[test]
//     pub fn test_chunk_type_is_safe_to_copy() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_safe_to_copy());
//     }

//     #[test]
//     pub fn test_chunk_type_is_unsafe_to_copy() {
//         let chunk = ChunkType::from_str("RuST").unwrap();
//         assert!(!chunk.is_safe_to_copy());
//     }

//     #[test]
//     pub fn test_valid_chunk_is_valid() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert!(chunk.is_valid());
//     }

//     #[test]
//     pub fn test_invalid_chunk_is_valid() {
//         let chunk = ChunkType::from_str("Rust").unwrap();
//         assert!(!chunk.is_valid());

//         let chunk = ChunkType::from_str("Ru1t");
//         assert!(chunk.is_err());
//     }

//     #[test]
//     pub fn test_chunk_type_string() {
//         let chunk = ChunkType::from_str("RuSt").unwrap();
//         assert_eq!(&chunk.to_string(), "RuSt");
//     }

//     #[test]
//     pub fn test_chunk_type_trait_impls() {
//         let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
//         let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
//         let _chunk_string = format!("{}", chunk_type_1);
//         let _are_chunks_equal = chunk_type_1 == chunk_type_2;
//     }
// }
