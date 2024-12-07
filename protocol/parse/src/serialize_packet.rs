use crate::prelude::{EncodePacket, VarInt};
use crate::var_int::{CONTINUE_BIT, SEGMENT_BITS};
use uuid::Uuid;

pub trait SerializePacketData: Sized {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
}

impl SerializePacketData for VarInt {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut value = self.value();

        loop {
            if (value & !(SEGMENT_BITS as i32)) == 0 {
                bytes.push(value as u8);
                break;
            }

            bytes.push(((value & SEGMENT_BITS as i32) as u8) | CONTINUE_BIT);
            value = (value as u32 >> 7) as i32;
        }

        Ok(())
    }
}

impl SerializePacketData for String {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        VarInt::new(self.len() as i32).encode(bytes)?;
        bytes.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl SerializePacketData for i64 {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        bytes.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl SerializePacketData for Uuid {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        bytes.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl SerializePacketData for bool {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        bytes.push(if *self { 0x01 } else { 0x00 });
        Ok(())
    }
}

impl<T: EncodePacket> SerializePacketData for T {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.encode()?.iter().for_each(|byte| bytes.push(*byte));
        Ok(())
    }
}

impl<T: SerializePacketData> SerializePacketData for Option<T> {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(value) = self {
            value.encode(bytes)?
        }
        Ok(())
    }
}

impl<T: SerializePacketData> SerializePacketData for Vec<T> {
    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.iter().for_each(|value| value.encode(bytes).unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::SerializePacketData;

    #[test]
    fn test_encode_string() {
        let mut bytes = Vec::new();
        "hello".to_string().encode(&mut bytes).unwrap();
        assert_eq!(bytes, vec![5, 104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_encode_i64() {
        let mut bytes = Vec::new();
        123456789i64.encode(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0, 0, 0, 0, 7, 91, 205, 21]);
    }
}
