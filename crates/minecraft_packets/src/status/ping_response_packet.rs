use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn, PacketOut)]
#[packet_id("status/clientbound/minecraft:pong_response")]
pub struct PingResponsePacket {
    pub timestamp: i64,
}
