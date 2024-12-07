use crate::client::ClientReadError;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use protocol::prelude::{DecodePacket, PacketId};

pub enum StatusResult {
    Status,
    Ping(i64),
}

pub fn handle_status_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<StatusResult, Box<dyn std::error::Error>> {
    match packet_id {
        StatusRequestPacket::PACKET_ID => {
            StatusRequestPacket::decode(payload)?;
            Ok(StatusResult::Status)
        }
        PingRequestPacket::PACKET_ID => {
            let packet = PingRequestPacket::decode(payload)?;
            Ok(StatusResult::Ping(packet.timestamp))
        }
        _ => Err(Box::new(ClientReadError::UnknownPacket(packet_id))),
    }
}
