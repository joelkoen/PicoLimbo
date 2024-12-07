use protocol::parse::VarInt;
use protocol::*;

#[derive(Debug, Packet)]
pub struct HandshakePacket {
    pub protocol: VarInt,
    pub hostname: String,
    pub port: u16,
    pub next_state: VarInt,
}
