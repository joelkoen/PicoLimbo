use crate::packets::configuration::finish_configuration_packet::FinishConfigurationPacket;
use crate::packets::login::login_success_packet::LoginSuccessPacket;
use crate::packets::status::ping_response_packet::PingResponsePacket;
use crate::packets::status::status_response::StatusResponse;
use crate::packets::status::status_response_packet::StatusResponsePacket;
use crate::payload::{Payload, PayloadAppendError};
use crate::state::handle_configuration_state::{handle_configuration_state, ConfigurationResult};
use crate::state::handle_handshake_state::handle_handshake_state;
use crate::state::handle_login_state::{handle_login_state, LoginResult};
use crate::state::handle_status_state::{handle_status_state, StatusResult};
use crate::state::State;
use protocol::prelude::{EncodePacket, PacketId, SerializePacketData, VarInt};
use std::fmt::Debug;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{error, info};

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
    #[error("unknown packet received; state={0}, packet_id=0x{1:02x}")]
    UnknownPacket(State, u8),
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
                let result = handle_status_state(packet_id, packet_payload)?;
                match result {
                    StatusResult::Status => {
                        let packet = StatusResponsePacket::from_status_response(
                            &StatusResponse::new("1.21.4", 769, "A Minecraft Server", false),
                        );
                        self.write_packet(packet).await?;
                    }
                    StatusResult::Ping(timestamp) => {
                        let packet = PingResponsePacket { timestamp };
                        self.write_packet(packet).await?;
                    }
                };
                Ok(())
            }
            State::Login => {
                let result = handle_login_state(packet_id, packet_payload)?;
                match result {
                    LoginResult::Login(uuid, username) => {
                        let packet = LoginSuccessPacket {
                            uuid,
                            username,
                            number_of_properties: VarInt::new(0),
                            properties: Vec::new(),
                        };
                        self.write_packet(packet).await?;
                    }
                    LoginResult::LoginAcknowledged => {
                        self.update_state(State::Configuration);
                    }
                }
                Ok(())
            }
            State::Configuration => {
                let result = handle_configuration_state(packet_id, packet_payload)?;
                match result {
                    ConfigurationResult::Play => {
                        self.update_state(State::Play);
                        Ok(())
                    }
                    ConfigurationResult::Brand(brand) => {
                        info!("Client brand: {}", brand);
                        Ok(())
                    }
                    ConfigurationResult::ClientInformation => {
                        let packet = FinishConfigurationPacket {};
                        self.write_packet(packet).await?;
                        Ok(())
                    }
                }
            }
            State::Play => Err(Box::new(ClientReadError::NotSupportedState(State::Play))),
            State::Transfer => Err(Box::new(ClientReadError::NotSupportedState(
                State::Transfer,
            ))),
        }
    }

    async fn write_packet(
        &mut self,
        packet: impl EncodePacket + PacketId,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encoded_packet = packet.encode()?;
        let mut payload = Vec::new();
        VarInt::new(encoded_packet.len() as i32 + 1).encode(&mut payload)?;
        payload.push(packet.get_packet_id());
        payload.extend_from_slice(&encoded_packet);

        self.socket.write_all(&payload).await?;
        Ok(())
    }
}
