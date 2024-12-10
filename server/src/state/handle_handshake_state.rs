use crate::packet_error::PacketError;
use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

/// Returns the next state
pub fn handle_handshake_state(packet_id: u8, payload: &[u8]) -> Result<State, PacketError> {
    match packet_id {
        HandshakePacket::PACKET_ID => {
            let packet = HandshakePacket::decode(payload)?;
            Ok(packet.get_next_state().map_err(|_| PacketError::Decode)?)
        }
        _ => Err(PacketError::new(State::Handshake, packet_id)),
    }
}
