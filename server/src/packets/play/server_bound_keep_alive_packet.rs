use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x1A, "play/server/minecraft:keep_alive")]
pub struct ServerBoundKeepAlivePacket {
    id: i64,
}
