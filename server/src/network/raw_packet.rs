use protocol::prelude::{EncodePacket, PacketId};

#[derive(Debug)]
pub struct RawPacket {
    data: Vec<u8>,
}

impl RawPacket {
    /// Creates a raw packet, containing its ID and associated data.
    /// The data vector must not be length padded.
    pub fn new(data: Vec<u8>) -> Self {
        RawPacket { data }
    }

    /// Creates a new raw packet from a serializable packet struct.
    pub fn from_packet<T>(packet: T) -> anyhow::Result<Self>
    where
        T: EncodePacket + PacketId,
    {
        let mut data = vec![T::PACKET_ID];
        data.extend_from_slice(&packet.encode()?);
        Ok(Self { data })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn packet_id(&self) -> u8 {
        self.data[0]
    }

    pub fn data(&self) -> &[u8] {
        &self.data[1..]
    }
}

impl From<Vec<u8>> for RawPacket {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}
