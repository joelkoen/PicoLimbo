pub trait DecodePacketField: Sized {
    type Error: std::error::Error;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error>;
}
