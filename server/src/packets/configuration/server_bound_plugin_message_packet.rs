use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x02, "configuration/serverbound/minecraft:custom_payload")]
pub struct ServerBoundPluginMessagePacket {
    channel: Identifier,
    data: LengthPaddedVec<i8>,
}
