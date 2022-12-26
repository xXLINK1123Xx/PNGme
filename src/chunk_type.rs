use std::{str::FromStr, fmt::Display};

use crate::{Error};

/*
	My mistakes: 
		- Error handling
		- Flag checking

	12 of 14 tests were passed in the start
*/

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType([u8; 4]);


impl TryFrom<[u8; 4]> for ChunkType {

    fn try_from(value: [u8; 4]) -> Result<Self, self::Error> {
        Ok(Self(value))
    }

    type Error = Error;
}

impl FromStr for ChunkType {

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars: [u8; 4] = [0,0,0,0];

        if s.len() != 4 { return Err(Box::new(ChunkTypeError::ByteLengthError(s.len()))); }

		let valid_chars = s.as_bytes()
		.iter()
		.all(|&c| c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z');

		if !valid_chars {
			return Err(Box::new(ChunkTypeError::InvalidCharacter));
		}

		let mut i = 0;
		for &b in s.as_bytes() {
			chars[i] = b;
			i +=1;
		}

        Ok(Self(chars))
    }

    type Err = Error;
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0  {
			write!(f, "{}", c as char);
		}
		Ok(())
    }
}

impl ChunkType {
	pub fn bytes(&self) -> [u8; 4] {
		self.0
	}

	pub fn is_valid(&self) -> bool {
		let valid_chars =
		self.0
		.iter()
		.all(|&c| c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z');

		self.is_reserved_bit_valid() && valid_chars
	}

	pub fn is_critical(&self) -> bool {
		(self.0[0] & 0x20) != 0x20
	}

	pub fn is_public(&self) -> bool {
		(self.0[1] & 0x20) != 0x20
	}

	pub fn is_reserved_bit_valid(&self) -> bool {
		(self.0[2] & 0x20) != 0x20
	}

	pub fn is_safe_to_copy(&self) -> bool {
		(self.0[3] & 0x20) == 0x20
	}
}
/// Chunk type errors
#[derive(Debug)]
pub enum ChunkTypeError {
    /// Chunk has incorrect number of bytes (4 expected)
    ByteLengthError(usize),

    /// The input string contains an invalid character at the given index
    InvalidCharacter,
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes but received {} when creating chunk type",
                actual
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Input contains one or more invalid characters")
            }
        }
    }
}

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