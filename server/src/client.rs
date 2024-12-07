use crate::packets::handshake_packet::HandshakePacket;
use crate::payload::{Payload, PayloadAppendError};
use crate::state::State;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::AsyncReadExt;
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
    #[error("invalid packet received; error={0}")]
    InvalidPacket(PayloadAppendError),
    #[error("no bytes received from the client")]
    NoBytesReceived,
    #[error("failed to read socket; error={0}")]
    FailedToRead(std::io::Error),
    #[error("unknown packet {0}")]
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

    pub fn is_handshaking(&self) -> bool {
        self.state == State::Handshake
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

    pub fn handle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.get_payload().get_data();
        let packet_id = bytes[0];
        let packet_payload = &bytes[1..];

        match self.state {
            State::Handshake => {
                let next_state = handle_handshake_state(packet_id, packet_payload)?;
                self.update_state(next_state);
                Ok(())
            }
            State::Status => Err(Box::new(ClientReadError::NotSupportedState(State::Status))),
            State::Login => Err(Box::new(ClientReadError::NotSupportedState(State::Login))),
            State::Transfer => Err(Box::new(ClientReadError::NotSupportedState(
                State::Transfer,
            ))),
        }
    }
}

/// Returns the next state
fn handle_handshake_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<State, Box<dyn std::error::Error>> {
    match packet_id {
        0x00 => {
            let handshake_packet = HandshakePacket::parse(payload)?;
            debug!("handshake packet: {:?}", handshake_packet);
            Ok(State::parse(handshake_packet.next_state.value())?)
        }
        _ => {
            error!("unknown packet id: {}", packet_id);
            Err(Box::new(ClientReadError::UnknownPacket(packet_id)))
        }
    }
}
