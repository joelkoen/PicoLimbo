use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x00, "status/serverbound/minecraft:status_request")]
pub struct StatusRequestPacket {}
