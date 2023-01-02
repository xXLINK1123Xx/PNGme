use std::fmt::Display;

use crc::Crc;

use crate::chunk_type::{ChunkType, self};

use crate::{Result, Error};

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
	chunk_type: ChunkType,
	data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let chunk_data: Vec<u8> = []
        .iter()
        .chain(chunk_type.bytes().iter())
        .chain(data.iter())
        .copied()
        .collect();

		Self {
            chunk_type,
            length: data.len() as u32,
            crc: crc.checksum(&chunk_data),
            data
        }
	}

    pub fn length(&self) -> u32 {
		self.length
	}

    pub fn chunk_type(&self) -> &ChunkType {
		&self.chunk_type
	}

    pub fn data(&self) -> &[u8] {
		&self.data
	}

    pub fn crc(&self) -> u32 {
		self.crc
	}

    pub fn data_as_string(&self) -> Result<String> {
        Ok(self.data.clone().iter().map(|&c| c as char).collect())
	}

    pub fn as_bytes(&self) -> Vec<u8> {
		self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
	}
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Box::new(ChunkError::ByteLengthError(value.len())));
        }

        let data_length = u32::from_be_bytes(value[0..4].try_into().unwrap());
        let chunk_type_bytes: [u8; 4] = value[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();
        let chunk_data: Vec<u8> = value[8..(data_length as usize + 8)].to_vec();
        let crc = u32::from_be_bytes(value[(data_length as usize + 8)..(data_length as usize + 12)].try_into().unwrap());
        let crc_alg = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let actual_crc = crc_alg.checksum(&value[4..(data_length as usize + 8)]);
        if crc != actual_crc {
            return Err(Box::new(ChunkError::MismatchedCrcError(actual_crc)));
        }

        Ok(Self {
            length: data_length,
            chunk_type: chunk_type,
            data: chunk_data,
            crc: crc
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

/// Chunk type errors
#[derive(Debug)]
pub enum ChunkError {
    /// Chunk has incorrect number of bytes (4 expected)
    ByteLengthError(usize),

    /// The input string contains an invalid character at the given index
    MismatchedCrcError(u32),
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes but received {} when creating chunk type",
                actual
            ),
            ChunkError::MismatchedCrcError(actual) => {
                write!(f, "Provided crc does not match actual chuck's data crc: {}", actual)
            }
        }
    }
}

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