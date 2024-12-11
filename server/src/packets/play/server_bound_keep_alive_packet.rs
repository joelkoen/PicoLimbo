use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x1A)]
pub struct ServerBoundKeepAlivePacket {
    id: i64,
}
