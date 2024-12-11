use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x0B)]
pub struct ClientTickEndPacket {}
