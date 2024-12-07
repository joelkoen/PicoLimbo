use crate::prelude::DeserializePacketData;
use crate::var_int::{CONTINUE_BIT, VarInt};
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("invalid string size error")]
pub struct InvalidStringSizeError;

pub fn read_string(bytes: &[u8], index: &mut usize) -> Result<String, Box<dyn Error>> {
    let length = VarInt::decode(bytes, index)?.value() as usize;

    if length > 255 {
        return Err(Box::new(InvalidStringSizeError));
    }

    while (bytes[*index] & CONTINUE_BIT) != 0 {
        *index += 1;
    }

    let result = std::str::from_utf8(&bytes[*index..*index + length])?;

    *index += length;

    Ok(result.to_string())
}
