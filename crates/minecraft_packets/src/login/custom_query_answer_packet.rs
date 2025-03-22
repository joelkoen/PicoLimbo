use minecraft_protocol::prelude::*;

/// This packet is currently only used to communicate with the Velocity proxy.
#[derive(Debug, PacketIn)]
#[packet_id("login/serverbound/minecraft:custom_query_answer")]
pub struct CustomQueryAnswerPacket {
    pub message_id: VarInt,
    pub is_present: bool,
    pub data: Vec<u8>,
}
