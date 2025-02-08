pub trait EncodePacket: Sized {
    fn encode(&self) -> anyhow::Result<Vec<u8>>;
}
