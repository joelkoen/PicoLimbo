use crate::packets::login::login_acknowledged_packet::LoginAcknowledgedPacket;
use crate::packets::login::login_state_packet::LoginStartPacket;
use crate::packets::login::login_success_packet::LoginSuccessPacket;
use crate::server::SharedClient;
use crate::state::State;

pub async fn on_login_start(client: SharedClient, packet: LoginStartPacket) {
    let packet = LoginSuccessPacket {
        uuid: packet.player_uuid,
        username: packet.name,
        properties: Vec::new().into(),
    };
    client.lock().await.send_packet(packet).await;
}

pub async fn on_login_acknowledged(client: SharedClient, _packet: LoginAcknowledgedPacket) {
    client.lock().await.update_state(State::Configuration);
}
