use crate::packet_error::PacketError;
use crate::packet_handler::PacketHandler;
use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::state::State;

/// Returns the next state
pub fn handle_handshake_state(packet_id: u8, payload: &[u8]) -> Result<State, PacketError> {
    PacketHandler::new(State::Handshake)
        .on::<HandshakePacket>(|packet| packet.get_next_state().map_err(|_| PacketError::Decode))
        .handle(packet_id, payload)?
}
