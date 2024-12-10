use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x1A)]
pub struct ServerBoundKeepAlivePacket {
    id: i64,
}

impl ServerBoundKeepAlivePacket {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}
