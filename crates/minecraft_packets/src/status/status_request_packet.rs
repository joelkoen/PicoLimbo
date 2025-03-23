use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn, PacketOut)]
#[packet_id("status/serverbound/minecraft:status_request")]
pub struct StatusRequestPacket {}

impl StatusRequestPacket {
    pub fn new() -> Self {
        Self {}
    }
}
