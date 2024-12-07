use protocol::prelude::*;

#[derive(Debug, PacketIn)]
pub struct PingRequestPacket {
    pub timestamp: i64,
}
