use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x01)]
pub struct PingRequestPacket {
    pub timestamp: i64,
}
