use crate::client::Client;
use crate::ping_server::ping_server;
use crate::server_manager::ServerManager;
use minecraft_packets::handshaking::handshake_packet::HandshakePacket;
use minecraft_protocol::protocol_version::ProtocolVersion;
use minecraft_protocol::state::State;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, error};

pub async fn on_handshake(
    client: &mut Client,
    packet: &HandshakePacket,
    backend_server_address: &str,
    server_manager: &Arc<Mutex<ServerManager>>,
) {
    client.set_protocol(packet.get_protocol());

    if let Ok(state) = packet.get_next_state() {
        if state == State::Login {
            client.set_wants_to_login(true);
        }

        client.update_state(state);
    }

    if let Err(err) = ping_server(packet, backend_server_address).await {
        error!("Ping server error: {}", err);
        if let Ok(state) = packet.get_next_state() {
            if state == State::Login {
                if let Err(err) = server_manager.lock().await.start_server().await {
                    error!("Failed to start server: {}", err);
                }
            }
        }
    } else {
        debug!("Backend server is available: {}", backend_server_address);
        client.set_backend_server_available(packet.clone());
    }
}

#[derive(Error, Debug)]
#[error("unknown state {0}")]
pub struct UnknownStateError(i32);

pub trait GetStateProtocol {
    fn get_next_state(&self) -> Result<State, UnknownStateError>;
    fn get_protocol(&self) -> ProtocolVersion;
}

impl GetStateProtocol for HandshakePacket {
    fn get_next_state(&self) -> Result<State, UnknownStateError> {
        let state = self.next_state.value();
        match state {
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            3 => Ok(State::Transfer),
            _ => Err(UnknownStateError(state)),
        }
    }

    fn get_protocol(&self) -> ProtocolVersion {
        ProtocolVersion::from(self.protocol.value())
    }
}
