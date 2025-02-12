use crate::handlers::configuration::send_play_packets;
use crate::server::client::SharedClient;
use crate::server::game_profile::GameProfile;
use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use minecraft_packets::login::game_profile_packet::GameProfilePacket;
use minecraft_packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use minecraft_packets::login::login_state_packet::LoginStartPacket;
use minecraft_packets::login::login_success_packet::LoginSuccessPacket;
use tracing::info;

pub async fn on_login_start(client: SharedClient, packet: LoginStartPacket) {
    let game_profile: GameProfile = packet.into();
    let mut client = client.lock().await;

    if client.protocol_version() >= ProtocolVersion::V1_21_2 {
        let packet = LoginSuccessPacket {
            uuid: game_profile.uuid(),
            username: game_profile.username().to_string(),
            properties: Vec::new().into(),
        };
        client.send_packet(packet).await;
    } else {
        let packet = GameProfilePacket {
            uuid: game_profile.uuid(),
            username: game_profile.username().to_string(),
            properties: Vec::new().into(),
            strict_error_handling: false,
        };
        client.send_packet(packet).await;
    }

    client.set_game_profile(game_profile.clone());
    info!(
        "UUID of player {} is {}",
        game_profile.username(),
        game_profile.uuid()
    );

    if client.protocol_version() < ProtocolVersion::V1_20_2 {
        send_play_packets(client).await;
    }
}

pub async fn on_login_acknowledged(client: SharedClient, _packet: LoginAcknowledgedPacket) {
    let mut client = client.lock().await;
    if client.protocol_version() >= ProtocolVersion::V1_20_2 {
        client.update_state(State::Configuration);
    }
}
