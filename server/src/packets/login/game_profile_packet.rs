use crate::packets::login::login_success_packet::Property;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("login/clientbound/minecraft:game_profile")]
pub struct GameProfilePacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: LengthPaddedVec<Property>,
    #[pvn(766..)]
    pub strict_error_handling: bool,
}
