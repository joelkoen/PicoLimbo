use crate::packet_error::PacketError;
use crate::packet_handler::PacketHandler;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use crate::state::State;

pub enum StatusResult {
    Status,
    Ping(i64),
}

pub fn handle_status_state(packet_id: u8, payload: &[u8]) -> Result<StatusResult, PacketError> {
    PacketHandler::new(State::Status)
        .on::<StatusRequestPacket>(|_| StatusResult::Status)
        .on::<PingRequestPacket>(|packet| StatusResult::Ping(packet.timestamp))
        .handle(packet_id, payload)
}
