pub use crate::var_int::{VarInt, VarIntParseError};

mod data_types;
mod var_int;

pub trait Parse: Sized {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>>;
}

impl Parse for VarInt {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(VarInt::parse(bytes, index)?)
    }
}

impl Parse for String {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        data_types::string::read_string(bytes, index)
    }
}

impl Parse for u16 {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(data_types::unsigned_short::read_unsigned_short(
            bytes, index,
        ))
    }
}
