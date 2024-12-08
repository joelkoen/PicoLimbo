use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x01)]
pub struct ClientBoundPluginMessagePacket {
    channel: Identifier,
    data: Vec<i8>,
}

impl ClientBoundPluginMessagePacket {
    pub fn brand(brand: impl ToString) -> Self {
        Self {
            channel: Identifier::minecraft("brand"),
            data: brand
                .to_string()
                .as_bytes()
                .iter()
                .map(|&b| b as i8)
                .collect(),
        }
    }
}
