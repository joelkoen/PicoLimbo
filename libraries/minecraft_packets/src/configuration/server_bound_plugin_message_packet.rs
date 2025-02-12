use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("configuration/serverbound/minecraft:custom_payload")]
pub struct ServerBoundPluginMessagePacket {
    channel: Identifier,
    data: LengthPaddedVec<i8>,
}
