use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x1A, "play/serverbound/minecraft:keep_alive")]
pub struct ServerBoundKeepAlivePacket {
    id: i64,
}
