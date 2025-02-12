use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id("login/serverbound/minecraft:login_acknowledged")]
pub struct LoginAcknowledgedPacket {}
