use crate::client::Client;
use crate::server_manager::{ServerManager, ServerStatus};
use minecraft_packets::status::data::status_response::StatusResponse;
use minecraft_packets::status::ping_request_packet::PingRequestPacket;
use minecraft_packets::status::ping_response_packet::PingResponsePacket;
use minecraft_packets::status::status_request_packet::StatusRequestPacket;
use minecraft_packets::status::status_response_packet::StatusResponsePacket;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_status_request(
    client: &mut Client,
    _packet: &StatusRequestPacket,
    server_manager: &Arc<Mutex<ServerManager>>,
) {
    if client.is_backend_server_available() {
        panic!("Why isn't this proxying?")
    }

    let server_manager = server_manager.lock().await;
    let (version_name, version_protocol) = match server_manager.get_server_status().await {
        ServerStatus::Offline => ("§5Sleeping", 0),
        ServerStatus::Starting => ("§6Starting…", 0),
        ServerStatus::Online => ("§4Error", 0),
    };

    let status_response =
        StatusResponse::new(version_name, version_protocol, "A Minecraft Server", false);
    let packet = StatusResponsePacket::from_status_response(&status_response);
    client.send_packet(packet).await;
}

pub async fn on_ping_request(client: &mut Client, packet: &PingRequestPacket) {
    let packet = PingResponsePacket {
        timestamp: packet.timestamp,
    };
    client.send_packet(packet).await;
}
