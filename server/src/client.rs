use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::packets::login::login_state_packet::LoginStartPacket;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::ping_response_packet::PingResponsePacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use crate::packets::status::status_response::StatusResponse;
use crate::packets::status::status_response_packet::StatusResponsePacket;
use crate::payload::{Payload, PayloadAppendError};
use crate::state::State;
use protocol::prelude::{DecodePacket, EncodePacket, SerializePacketData, VarInt};
use std::fmt::Debug;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error};

pub struct Client {
    socket: TcpStream,
    state: State,
    payload: Payload,
    address: SocketAddr,
}

#[derive(Error, Debug)]
pub enum ClientReadError {
    #[error("invalid packet_in received; error={0}")]
    InvalidPacket(PayloadAppendError),
    #[error("no bytes received from the client")]
    NoBytesReceived,
    #[error("failed to read socket; error={0}")]
    FailedToRead(std::io::Error),
    #[error("unknown packet_in {0}")]
    UnknownPacket(u8),
    #[error("state not supported {0}")]
    NotSupportedState(State),
}

impl Client {
    pub fn new(socket: TcpStream, address: SocketAddr) -> Client {
        Client {
            socket,
            address,
            state: State::Handshake,
            payload: Payload::new(),
        }
    }

    pub fn update_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub async fn read_socket(&mut self) -> Result<(), ClientReadError> {
        let mut buf = vec![0; self.payload.get_remaining_to_read()];

        let bytes_received = self
            .socket
            .read(&mut buf)
            .await
            .map_err(ClientReadError::FailedToRead)?;

        if bytes_received == 0 {
            return Err(ClientReadError::NoBytesReceived);
        }

        if let Err(err) = self
            .payload
            .append_bytes(&buf[..bytes_received], bytes_received)
        {
            return Err(ClientReadError::InvalidPacket(err));
        }

        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.payload.is_complete()
    }

    pub fn get_payload(&self) -> &Payload {
        &self.payload
    }

    pub fn reset_payload(&mut self) -> Result<(), PayloadAppendError> {
        self.payload.reset()
    }

    pub async fn handle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.get_payload().get_data();
        let packet_id = bytes[0];
        let packet_payload = &bytes[1..];

        match self.state {
            State::Handshake => {
                let next_state = handle_handshake_state(packet_id, packet_payload)?;
                self.update_state(next_state);
                Ok(())
            }
            State::Status => {
                let result = handle_status(packet_id, packet_payload)?;
                match result {
                    StatusResult::Status => {
                        let packet = StatusResponsePacket::from_status_response(
                            &StatusResponse::new("1.21.4", 769, "A Minecraft Server", false),
                        );
                        self.write_packet(0x00, packet).await?;
                    }
                    StatusResult::Ping(timestamp) => {
                        let packet = PingResponsePacket { timestamp };
                        self.write_packet(0x01, packet).await?;
                    }
                };

                Ok(())
            }
            State::Login => {
                handle_login(packet_id, packet_payload)?;
                Ok(())
            }
            State::Transfer => Err(Box::new(ClientReadError::NotSupportedState(
                State::Transfer,
            ))),
        }
    }

    async fn write_packet(
        &mut self,
        packet_id: u8,
        packet: impl EncodePacket,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encoded_packet = packet.encode()?;
        let mut payload = Vec::new();
        VarInt::new(encoded_packet.len() as i32 + 1).encode(&mut payload)?;
        payload.push(packet_id);
        payload.extend_from_slice(&encoded_packet);

        self.socket.write_all(&payload).await?;
        Ok(())
    }
}

/// Returns the next state
fn handle_handshake_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<State, Box<dyn std::error::Error>> {
    match packet_id {
        0x00 => {
            let packet = HandshakePacket::decode(payload)?;
            debug!("{:?}", packet);
            Ok(State::parse(packet.next_state.value())?)
        }
        _ => {
            error!("unknown packet_in id: {}", packet_id);
            Err(Box::new(ClientReadError::UnknownPacket(packet_id)))
        }
    }
}

enum StatusResult {
    Status,
    Ping(i64),
}

fn handle_status(
    packet_id: u8,
    payload: &[u8],
) -> Result<StatusResult, Box<dyn std::error::Error>> {
    match packet_id {
        0x00 => {
            let packet = StatusRequestPacket::decode(payload)?;
            debug!("{:?}", packet);
            Ok(StatusResult::Status)
        }
        0x01 => {
            let packet = PingRequestPacket::decode(payload)?;
            debug!("{:?}", packet);
            Ok(StatusResult::Ping(packet.timestamp))
        }
        _ => {
            error!("unknown packet_in id: {}", packet_id);
            Err(Box::new(ClientReadError::UnknownPacket(packet_id)))
        }
    }
}

fn handle_login(packet_id: u8, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match packet_id {
        0x00 => {
            let packet = LoginStartPacket::decode(payload)?;
            debug!("{:?}", packet);
            Ok(())
        }
        _ => {
            error!("unknown packet_in id: {}", packet_id);
            Err(Box::new(ClientReadError::UnknownPacket(packet_id)))
        }
    }
}
