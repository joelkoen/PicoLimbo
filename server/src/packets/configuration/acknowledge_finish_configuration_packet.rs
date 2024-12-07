use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x03)]
pub struct AcknowledgeConfigurationPacket {}
