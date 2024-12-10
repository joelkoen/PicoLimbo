use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x27)]
pub struct ClientBoundKeepAlivePacket {
    id: i64,
}

impl ClientBoundKeepAlivePacket {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}
