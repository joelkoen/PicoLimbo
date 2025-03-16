use minecraft_protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("login/serverbound/minecraft:custom_query_answer")]
pub struct CustomQueryAnswerPacket {
    pub message_id: VarInt,
    pub is_present: bool,
    pub data: Vec<u8>,
}
