use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("play/serverbound/minecraft:keep_alive")]
pub struct ServerBoundKeepAlivePacket {
    id: i64,
}
