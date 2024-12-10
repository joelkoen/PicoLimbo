use crate::packet_error::PacketError;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

pub enum StatusResult {
    Status,
    Ping(i64),
}

pub fn handle_status_state(packet_id: u8, payload: &[u8]) -> Result<StatusResult, PacketError> {
    match packet_id {
        StatusRequestPacket::PACKET_ID => {
            StatusRequestPacket::decode(payload)?;
            Ok(StatusResult::Status)
        }
        PingRequestPacket::PACKET_ID => {
            let packet = PingRequestPacket::decode(payload)?;
            Ok(StatusResult::Ping(packet.timestamp))
        }
        _ => Err(PacketError::new(State::Status, packet_id)),
    }
}
