use crate::data_types;
use crate::prelude::{Uuid, VarInt};

pub trait DeserializePacketData: Sized {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>>;
}

impl DeserializePacketData for VarInt {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(VarInt::parse(bytes, index)?)
    }
}

impl DeserializePacketData for String {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        data_types::string::read_string(bytes, index)
    }
}

impl DeserializePacketData for u16 {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(data_types::unsigned_short::read_unsigned_short(
            bytes, index,
        ))
    }
}

impl DeserializePacketData for i64 {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(data_types::long::read_long(bytes, index))
    }
}

impl DeserializePacketData for Uuid {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        data_types::uuid::read_uuid(bytes, index)
    }
}
