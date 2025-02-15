use minecraft_protocol::prelude::*;

#[derive(Debug, Clone, PacketIn, PacketOut)]
#[packet_id("handshake/serverbound/minecraft:intention")]
pub struct HandshakePacket {
    pub protocol: VarInt,
    pub hostname: String,
    pub port: u16,
    /// 1: Status, 2: Login, 3: Transfer
    pub next_state: VarInt,
}
