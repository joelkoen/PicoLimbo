use protocol::prelude::*;

#[derive(Debug, PacketOut)]
pub struct PingResponsePacket {
    pub timestamp: i64,
}
