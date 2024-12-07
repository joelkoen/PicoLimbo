use protocol::prelude::*;

#[derive(Debug, PacketIn)]
pub struct LoginStartPacket {
    pub name: String,
    pub player_uuid: Uuid,
}
