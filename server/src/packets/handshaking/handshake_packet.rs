use protocol::prelude::*;

#[derive(Debug, PacketIn)]
pub struct HandshakePacket {
    pub protocol: VarInt,
    pub hostname: String,
    pub port: u16,
    pub next_state: VarInt,
}
