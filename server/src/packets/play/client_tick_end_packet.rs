use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x0B, "play/serverbound/minecraft:client_tick_end")]
pub struct ClientTickEndPacket {}
