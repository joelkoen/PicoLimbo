use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x02)]
pub struct ServerBoundPluginMessagePacket {
    channel: Identifier,
    data: LengthPaddedVec<i8>,
}
