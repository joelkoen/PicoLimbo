use crate::prelude::VarInt;
use crate::var_int::{CONTINUE_BIT, SEGMENT_BITS};
use std::io::Write;

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
