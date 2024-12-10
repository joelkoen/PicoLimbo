use crate::packet_error::PacketError;
use crate::packets::play::server_bound_keep_alive_packet::ServerBoundKeepAlivePacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

pub enum PlayResult {
    KeepAlive,
}

pub fn handle_play_state(packet_id: u8, payload: &[u8]) -> Result<PlayResult, PacketError> {
    match packet_id {
        ServerBoundKeepAlivePacket::PACKET_ID => {
            ServerBoundKeepAlivePacket::decode(payload)?;
            Ok(PlayResult::KeepAlive)
        }
        _ => Err(PacketError::new(State::Play, packet_id)),
    }
}
