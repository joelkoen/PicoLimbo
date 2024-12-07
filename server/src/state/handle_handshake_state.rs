use crate::client::ClientReadError;
use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::state::state::State;
use protocol::prelude::{DecodePacket, PacketId};

/// Returns the next state
pub fn handle_handshake_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<State, Box<dyn std::error::Error>> {
    match packet_id {
        HandshakePacket::PACKET_ID => {
            let packet = HandshakePacket::decode(payload)?;
            Ok(packet.get_next_state()?)
        }
        _ => Err(Box::new(ClientReadError::UnknownPacket(packet_id))),
    }
}
