use crate::packets::status::data::status_response::StatusResponse;
use crate::packets::status::ping_request_packet::PingRequestPacket;
use crate::packets::status::ping_response_packet::PingResponsePacket;
use crate::packets::status::status_request_packet::StatusRequestPacket;
use crate::packets::status::status_response_packet::StatusResponsePacket;
use crate::server::client::SharedClient;

pub async fn on_status_request(client: SharedClient, _packet: StatusRequestPacket) {
    let mut client = client.lock().await;
    let version = client.protocol_version();
    let status_response = StatusResponse::new(
        &version.to_string(),
        version.version_number(),
        "A Minecraft Server",
        false,
    );
    let packet = StatusResponsePacket::from_status_response(&status_response);
    client.send_packet(packet).await;
}

pub async fn on_ping_request(client: SharedClient, packet: PingRequestPacket) {
    let packet = PingResponsePacket {
        timestamp: packet.timestamp,
    };
    client.lock().await.send_packet(packet).await;
}
