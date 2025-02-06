use crate::packet_error::PacketError;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use protocol::prelude::{DecodePacket, PacketId};

pub enum StatusResult {
    Status,
    Ping(i64),
}

pub fn handle_status_state(packet_id: u8, payload: &[u8]) -> Result<StatusResult, PacketError> {
    let handler = PacketHandler::new()
        .on::<StatusRequestPacket>(|packet| StatusResult::Status)
        .on::<PingRequestPacket>(|packet| StatusResult::Ping(packet.timestamp));
    handler.handle(packet_id, payload)

    /*
    // This is the previous code, that manually parses each packet with a match

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
     */
}

pub struct PacketHandler<R> {}

impl<Output> PacketHandler<Output> {
    pub fn new() -> Self {
        Self {}
    }

    pub fn on<T, F>(&mut self, func: F) -> Self
    where
        T: PacketId + DecodePacket,
        F: Fn(T) -> Output + Clone,
    {
        todo!()
    }

    pub fn handle(&self, packet_id: u8, payload: &[u8]) -> Result<Output, PacketError> {
        todo!()
    }
}
