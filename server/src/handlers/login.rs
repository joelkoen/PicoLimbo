use crate::packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use crate::packets::login::login_state_packet::LoginStartPacket;
use crate::packets::login::login_success_packet::LoginSuccessPacket;
use crate::server::client::SharedClient;
use crate::server::game_profile::GameProfile;
use crate::state::State;
use tracing::info;

pub async fn on_login_start(client: SharedClient, packet: LoginStartPacket) {
    let game_profile: GameProfile = packet.into();

    let packet = LoginSuccessPacket {
        uuid: game_profile.uuid(),
        username: game_profile.username().to_string(),
        properties: Vec::new().into(),
    };

    let mut client = client.lock().await;
    client.send_packet(packet).await;

    client.set_game_profile(game_profile.clone());
    info!(
        "UUID of player {} is {}",
        game_profile.username(),
        game_profile.uuid()
    );
}

pub async fn on_login_acknowledged(client: SharedClient, _packet: LoginAcknowledgedPacket) {
    client.lock().await.update_state(State::Configuration);
}
