pub trait DeserializePacketData: Sized {
    type Error: std::error::Error;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error>;
}

impl DeserializePacketData for u16 {
    type Error = std::convert::Infallible;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = ((bytes[*index] as u16) << 8) | (bytes[*index + 1] as u16);
        *index += 2;
        Ok(value)
    }
}

impl DeserializePacketData for i64 {
    type Error = std::convert::Infallible;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = ((bytes[*index] as i64) << 56)
            | ((bytes[*index + 1] as i64) << 48)
            | ((bytes[*index + 2] as i64) << 40)
            | ((bytes[*index + 3] as i64) << 32)
            | ((bytes[*index + 4] as i64) << 24)
            | ((bytes[*index + 5] as i64) << 16)
            | ((bytes[*index + 6] as i64) << 8)
            | (bytes[*index + 7] as i64);
        *index += 8;
        Ok(value)
    }
}

impl DeserializePacketData for i8 {
    type Error = std::convert::Infallible;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = bytes[*index] as i8;
        *index += 1;
        Ok(value)
    }
}

impl DeserializePacketData for u8 {
    type Error = std::convert::Infallible;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = bytes[*index];
        *index += 1;
        Ok(value)
    }
}

impl DeserializePacketData for bool {
    type Error = std::convert::Infallible;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = bytes[*index] == 0x01;
        *index += 1;
        Ok(value)
    }
}
