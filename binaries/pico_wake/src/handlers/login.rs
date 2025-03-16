use crate::client::Client;
use minecraft_packets::login::login_disconnect::LoginDisconnectPacket;
use minecraft_packets::login::login_state_packet::LoginStartPacket;

pub async fn on_login_start(client: &mut Client, packet: &LoginStartPacket) {
    let packet = LoginDisconnectPacket::text(format!(
        "Hello {}! This server was automatically started and will be online soon.",
        packet.name
    ))
    .unwrap_or(LoginDisconnectPacket::default());
    client.send_packet(packet).await;
}
