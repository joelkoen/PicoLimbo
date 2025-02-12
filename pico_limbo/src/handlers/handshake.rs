use crate::server::client::SharedClient;
use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use minecraft_packets::handshaking::handshake_packet::HandshakePacket;
use thiserror::Error;

pub async fn on_handshake(client: SharedClient, packet: HandshakePacket) {
    let mut client = client.lock().await;
    client.set_protocol(packet.get_protocol());

    if let Ok(state) = packet.get_next_state() {
        client.update_state(state);
    }
}

#[derive(Error, Debug)]
#[error("unknown state {0}")]
pub struct UnknownStateError(i32);

trait GetStateProtocol {
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
