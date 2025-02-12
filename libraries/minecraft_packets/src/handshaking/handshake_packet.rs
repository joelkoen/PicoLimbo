use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("handshake/serverbound/minecraft:intention")]
pub struct HandshakePacket {
    pub protocol: VarInt,
    hostname: String,
    port: u16,
    pub next_state: VarInt,
}
