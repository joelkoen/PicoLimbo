use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("play/serverbound/minecraft:client_tick_end")]
pub struct ClientTickEndPacket {}
