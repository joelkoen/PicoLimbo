use crate::packet_error::PacketError;
use crate::packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use crate::packets::login::login_state_packet::LoginStartPacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId, Uuid};

pub enum LoginResult {
    Login(Uuid, String),
    LoginAcknowledged,
}

pub fn handle_login_state(packet_id: u8, payload: &[u8]) -> Result<LoginResult, PacketError> {
    match packet_id {
        LoginStartPacket::PACKET_ID => {
            let packet = LoginStartPacket::decode(payload)?;
            Ok(LoginResult::Login(packet.player_uuid, packet.name))
        }
        LoginAcknowledgedPacket::PACKET_ID => {
            LoginAcknowledgedPacket::decode(payload)?;
            Ok(LoginResult::LoginAcknowledged)
        }
        _ => Err(PacketError::new(State::Login, packet_id)),
    }
}
