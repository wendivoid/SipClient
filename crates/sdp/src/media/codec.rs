use crate::Attribute;

use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

pub enum CodecType {
    PCMU,
    PCMA
}

impl CodecType {

    pub fn as_u8(&self) -> u8 {
        match self {
            CodecType::PCMU => 0,
            CodecType::PCMA => 8
        }
    }

    pub fn from_u8(input: u8) -> IoResult<CodecType> {
        match input {
            0 => Ok(CodecType::PCMU),
            8 => Ok(CodecType::PCMA),
            _ => Err(IoError::new(IoErrorKind::InvalidInput, "Unknown Media Codec"))
        }
    }
}

pub struct Codec {
    pub ty: CodecType,
    pub attributes: Vec<Attribute>
}
