use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x01, "status/client/minecraft:pong_response")]
pub struct PingResponsePacket {
    pub timestamp: i64,
}
