use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("status/serverbound/minecraft:ping_request")]
pub struct PingRequestPacket {
    pub timestamp: i64,
}
