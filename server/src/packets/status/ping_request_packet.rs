use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x01, "status/serverbound/minecraft:ping_request")]
pub struct PingRequestPacket {
    pub timestamp: i64,
}
