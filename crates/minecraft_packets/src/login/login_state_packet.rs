use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("login/serverbound/minecraft:hello")]
pub struct LoginStartPacket {
    pub name: String,
    pub player_uuid: Uuid,
}
