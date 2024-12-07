use crate::data_types;
use crate::data_types::identifier::Identifier;
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
        let value = ((bytes[*index] as u16) << 8) | (bytes[*index + 1] as u16);
        *index += 2;
        Ok(value)
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

impl DeserializePacketData for Identifier {
    /// Decodes an identifier.
    /// An identifier is a String with a namespace and a path separated by a colon.
    /// If the namespace is not provided, it defaults to "minecraft".
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        let decoded_string = String::decode(bytes, index)?;
        Ok(Identifier::from_string(&decoded_string))
    }
}

impl DeserializePacketData for i8 {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        let value = bytes[*index] as i8;
        *index += 1;
        Ok(value)
    }
}

impl DeserializePacketData for u8 {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        let value = bytes[*index];
        *index += 1;
        Ok(value)
    }
}

impl DeserializePacketData for bool {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        let value = bytes[*index] == 0x01;
        *index += 1;
        Ok(value)
    }
}

impl<T: DeserializePacketData> DeserializePacketData for Vec<T> {
    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        let length = VarInt::decode(bytes, index)?.value();
        let mut vec = Vec::with_capacity(length as usize);

        for _ in 0..length {
            vec.push(T::decode(bytes, index)?);
        }

        Ok(vec)
    }
}
