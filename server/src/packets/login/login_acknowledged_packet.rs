use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x03, "login/server/minecraft:login_acknowledged")]
pub struct LoginAcknowledgedPacket {}
