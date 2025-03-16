use minecraft_protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("login/clientbound/minecraft:custom_query")]
pub struct CustomQueryPacket {
    pub message_id: VarInt,
    pub channel: Identifier,
    pub data: Vec<u8>,
}

impl CustomQueryPacket {
    pub fn velocity_info_channel(message_id: i32) -> Self {
        Self {
            message_id: VarInt::new(message_id),
            channel: Identifier::new("velocity", "player_info"),
            data: Vec::new(),
        }
    }
}
