use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x01)]
pub struct PingResponsePacket {
    pub timestamp: i64,
}
