use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x01, "status/clientbound/minecraft:pong_response")]
pub struct PingResponsePacket {
    pub timestamp: i64,
}
