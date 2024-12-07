use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x00)]
pub struct StatusRequestPacket {}
