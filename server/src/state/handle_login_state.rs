use crate::packet_error::PacketError;
use crate::packet_handler::PacketHandler;
use crate::packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use crate::packets::login::login_state_packet::LoginStartPacket;
use crate::state::State;
use protocol::prelude::Uuid;

pub enum LoginResult {
    Login(Uuid, String),
    LoginAcknowledged,
}

pub fn handle_login_state(packet_id: u8, payload: &[u8]) -> Result<LoginResult, PacketError> {
    PacketHandler::new(State::Login)
        .on::<LoginStartPacket>(|packet| LoginResult::Login(packet.player_uuid, packet.name))
        .on::<LoginAcknowledgedPacket>(|_| LoginResult::LoginAcknowledged)
        .handle(packet_id, payload)
}
