use minecraft_packets::configuration::client_bound_known_packs_packet::ClientBoundKnownPacksPacket;
use minecraft_packets::configuration::client_bound_plugin_message_packet::ClientBoundPluginMessagePacket;
use minecraft_packets::configuration::finish_configuration_packet::FinishConfigurationPacket;
use minecraft_packets::configuration::registry_data_packet::{
    RegistryDataCodecPacket, RegistryDataPacket,
};
use minecraft_packets::play::chunk_data_and_update_light_packet::ChunkDataAndUpdateLightPacket;
use minecraft_packets::play::game_event_packet::GameEventPacket;
use minecraft_packets::play::login_packet::LoginPacket;
use minecraft_packets::play::set_default_spawn_position::SetDefaultSpawnPosition;
use minecraft_packets::play::synchronize_player_position_packet::SynchronizePlayerPositionPacket;
use minecraft_protocol::data::registry::get_all_registries::{
    get_grouped_registries, get_registry_codec,
};
use minecraft_protocol::protocol_version::ProtocolVersion;
use minecraft_protocol::state::State;
use minecraft_server::client::Client;
use tokio::sync::MutexGuard;

pub async fn send_configuration_packets(mut client: MutexGuard<'_, Client>) {
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

    send_play_packets(client).await;
}

/// Switch to the Play state and send required packets to spawn the player in the world
pub async fn send_play_packets(mut client: MutexGuard<'_, Client>) {
    client.update_state(State::Play);

    let registry_codec = get_registry_codec(client.protocol_version());
    let dimension = registry_codec
        .find_tag("minecraft:dimension_type")
        .unwrap()
        .find_tag("value")
        .unwrap()
        .get_vec()
        .unwrap();

    let dimension = dimension
        .first()
        .unwrap()
        .find_tag("element")
        .unwrap()
        .clone();

    let packet = LoginPacket {
        registry_codec,
        dimension,
        ..Default::default()
    };
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
