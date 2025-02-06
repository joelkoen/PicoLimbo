use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x0B, "play/server/minecraft:client_tick_end")]
pub struct ClientTickEndPacket {}
