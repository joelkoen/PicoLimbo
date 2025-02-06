use crate::packet_error::PacketError;
use crate::packet_handler::PacketHandler;
use crate::packets::play::client_tick_end_packet::ClientTickEndPacket;
use crate::packets::play::server_bound_keep_alive_packet::ServerBoundKeepAlivePacket;
use crate::packets::play::set_player_position_packet::{
    SetPlayerPositionAndRotationPacket, SetPlayerPositionPacket, SetPlayerRotationPacket,
};
use crate::state::State;

pub enum PlayResult {
    Nothing,
}

pub fn handle_play_state(packet_id: u8, payload: &[u8]) -> Result<PlayResult, PacketError> {
    PacketHandler::new(State::Play)
        .on::<ServerBoundKeepAlivePacket>(|_| PlayResult::Nothing)
        .on::<ClientTickEndPacket>(|_| PlayResult::Nothing)
        .on::<SetPlayerPositionAndRotationPacket>(|_| PlayResult::Nothing)
        .on::<SetPlayerRotationPacket>(|_| PlayResult::Nothing)
        .on::<SetPlayerPositionPacket>(|_| PlayResult::Nothing)
        .handle(packet_id, payload)
}
