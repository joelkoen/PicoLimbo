pub trait DecodePacket: Sized {
    fn decode(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}
