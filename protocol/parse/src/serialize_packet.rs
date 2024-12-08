pub trait SerializePacketData: Sized {
    type Error: std::error::Error;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error>;
}

impl SerializePacketData for i64 {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        bytes.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl SerializePacketData for i32 {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        bytes.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl SerializePacketData for i8 {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        bytes.push(*self as u8);
        Ok(())
    }
}

impl SerializePacketData for u8 {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        bytes.push(*self);
        Ok(())
    }
}

impl SerializePacketData for bool {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        bytes.push(if *self { 0x01 } else { 0x00 });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::SerializePacketData;

    #[test]
    fn test_encode_i64() {
        let mut bytes = Vec::new();
        123456789i64.encode(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0, 0, 0, 0, 7, 91, 205, 21]);
    }
}
