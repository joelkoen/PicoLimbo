use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x02)]
pub struct ServerBoundPluginMessagePacket {
    pub channel: Identifier,
    data: Vec<i8>,
}
