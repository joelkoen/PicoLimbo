use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_bound_known_packs_packet::ClientBoundKnownPacksPacket;
use crate::packets::configuration::client_bound_plugin_message_packet::ClientBoundPluginMessagePacket;
use crate::packets::configuration::data::registry_entry::RegistryEntry;
use crate::packets::configuration::finish_configuration_packet::FinishConfigurationPacket;
use crate::packets::configuration::registry_data_packet::RegistryDataPacket;
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::packets::play::chunk_data_and_update_light_packet::ChunkDataAndUpdateLightPacket;
use crate::packets::play::game_event_packet::GameEventPacket;
use crate::packets::play::login_packet::LoginPacket;
use crate::packets::play::synchronize_player_position_packet::SynchronizePlayerPositionPacket;
use crate::registry::get_all_registries::get_all_registries;
use crate::server::client::SharedClient;
use crate::state::State;
use protocol::prelude::Identifier;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

pub async fn on_plugin_message(client: SharedClient, _packet: ServerBoundPluginMessagePacket) {
    let mut client = client.lock().await;

    // Send Server Brand
    let packet = ClientBoundPluginMessagePacket::brand("PicoLimbo");
    client.send_packet(packet).await;

    // Send Known Packs
    let packet = ClientBoundKnownPacksPacket::default();
    client.send_packet(packet).await;

    // Send Registry Data
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
    let data_dir = Path::new(&data_dir);
    let version_directory = data_dir.join("1_21_4").join("minecraft");
    let registries = get_all_registries(&version_directory);
    let registry_names = registries
        .iter()
        .map(|registry| registry.registry_id.clone())
        .collect::<HashSet<String>>();

    for registry_name in registry_names {
        let packet = RegistryDataPacket {
            registry_id: Identifier::from_str(&registry_name).unwrap(),
            entries: registries
                .iter()
                .filter(|entry| entry.registry_id == registry_name)
                .map(|entry| RegistryEntry {
                    entry_id: Identifier::minecraft(&entry.entry_id),
                    has_data: true,
                    nbt: Some(entry.nbt.clone()),
                })
                .collect::<Vec<_>>()
                .into(),
        };
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
    let mut client = client.lock().await;

    client.update_state(State::Play);

    let packet = LoginPacket::default();
    client.send_packet(packet).await;

    // Send Synchronize Player Position
    let packet = SynchronizePlayerPositionPacket::default();
    client.send_packet(packet).await;

    // Send Game Event
    let packet = GameEventPacket::start_waiting_for_chunks(0.0);
    client.send_packet(packet).await;

    // Send Chunk Data and Update Light
    let packet = ChunkDataAndUpdateLightPacket::default();
    client.send_packet(packet).await;

    client.send_keep_alive().await;
}
