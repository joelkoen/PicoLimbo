use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_bound_known_packs_packet::ClientBoundKnownPacksPacket;
use crate::packets::configuration::client_bound_plugin_message_packet::ClientBoundPluginMessagePacket;
use crate::packets::configuration::finish_configuration_packet::FinishConfigurationPacket;
use crate::packets::configuration::registry_data_packet::{
    RegistryDataCodecPacket, RegistryDataPacket,
};
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::packets::play::chunk_data_and_update_light_packet::ChunkDataAndUpdateLightPacket;
use crate::packets::play::game_event_packet::GameEventPacket;
use crate::packets::play::login_packet::LoginPacket;
use crate::packets::play::set_default_spawn_position::SetDefaultSpawnPosition;
use crate::packets::play::synchronize_player_position_packet::SynchronizePlayerPositionPacket;
use crate::registry::get_all_registries::{get_grouped_registries, get_registry_codec};
use crate::server::client::{Client, SharedClient};
use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use tokio::sync::MutexGuard;

pub async fn on_plugin_message(client: SharedClient, _packet: ServerBoundPluginMessagePacket) {
    let mut client = client.lock().await;

    // Send Server Brand
    let packet = ClientBoundPluginMessagePacket::brand("PicoLimbo");
    client.send_packet(packet).await;

    if client.protocol_version() >= ProtocolVersion::V1_20_5 {
        // Send Known Packs
        let packet = ClientBoundKnownPacksPacket::default();
        client.send_packet(packet).await;
    }

    // Send Registry Data
    if client.protocol_version() >= ProtocolVersion::V1_20_5 {
        let grouped = get_grouped_registries(client.protocol_version());
        for (registry_id, entries) in grouped {
            let packet = RegistryDataPacket {
                registry_id,
                entries: entries.into(),
            };
            client.send_packet(packet).await;
        }
    } else {
        let registry_codec = get_registry_codec(client.protocol_version()).to_nameless_compound();
        let packet = RegistryDataCodecPacket { registry_codec };
        client.send_packet(packet).await;
    }

    // Send Finished Configuration
    let packet = FinishConfigurationPacket {};
    client.send_packet(packet).await;
}

pub async fn on_acknowledge_configuration(
    client: SharedClient,
    _packet: AcknowledgeConfigurationPacket,
) {
    send_play_packets(client.lock().await).await;
}

pub async fn send_play_packets(mut client: MutexGuard<'_, Client>) {
    client.update_state(State::Play);

    let packet = LoginPacket::new(client.protocol_version());
    client.send_packet(packet).await;

    // Send Synchronize Player Position
    let packet = SynchronizePlayerPositionPacket::default();
    client.send_packet(packet).await;

    // Send Set Default Spawn Position
    let packet = SetDefaultSpawnPosition::default();
    client.send_packet(packet).await;

    if client.protocol_version() >= ProtocolVersion::V1_20_3 {
        // Send Game Event
        let packet = GameEventPacket::start_waiting_for_chunks(0.0);
        client.send_packet(packet).await;

        // Send Chunk Data and Update Light
        let packet = ChunkDataAndUpdateLightPacket::default();
        client.send_packet(packet).await;
    }

    client.send_keep_alive().await;
}
