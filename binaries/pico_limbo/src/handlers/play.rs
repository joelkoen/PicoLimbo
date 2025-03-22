use crate::ServerState;
use minecraft_packets::play::player_position::PlayerPositionPacket;
use minecraft_protocol::state::State;
use minecraft_server::client::SharedClient;

pub async fn on_player_position(
    _state: ServerState,
    client: SharedClient,
    _packet: PlayerPositionPacket,
) {
    let mut client = client.lock().await;
    client.update_state(State::Play);
    client.send_keep_alive().await;
}
