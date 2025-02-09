use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_bound_known_packs_packet::ClientBoundKnownPacksPacket;
use crate::packets::configuration::client_bound_plugin_message_packet::ClientBoundPluginMessagePacket;
use crate::packets::configuration::data::registry_entry::RegistryEntry;
use crate::packets::configuration::finish_configuration_packet::FinishConfigurationPacket;
use crate::packets::configuration::registry_data_packet::{
    RegistryDataCodecPacket, RegistryDataPacket,
};
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::packets::play::chunk_data_and_update_light_packet::ChunkDataAndUpdateLightPacket;
use crate::packets::play::game_event_packet::GameEventPacket;
use crate::packets::play::login_packet::LoginPacket;
use crate::packets::play::synchronize_player_position_packet::SynchronizePlayerPositionPacket;
use crate::registry::get_all_registries::get_all_registries;
use crate::server::client::SharedClient;
use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use protocol::prelude::{Identifier, Nbt};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

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
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data/generated".to_string());
    let version_directory = PathBuf::from(data_dir)
        .join(client.protocol_version().to_string())
        .join("data/minecraft");
    let registries = get_all_registries(&version_directory);

    let mut grouped: HashMap<Identifier, Vec<RegistryEntry>> = HashMap::new();

    for registry in &registries {
        let entry = RegistryEntry {
            entry_id: Identifier::minecraft(&registry.entry_id),
            has_data: true,
            nbt: Some(registry.nbt.clone()),
        };
        grouped
            .entry(Identifier::from_str(&registry.registry_id).unwrap())
            .or_default()
            .push(entry);
    }

    if client.protocol_version() >= ProtocolVersion::V1_20_5 {
        for (registry_id, entries) in grouped {
            let packet = RegistryDataPacket {
                registry_id,
                entries: entries.into(),
            };
            client.send_packet(packet).await;
        }
    } else {
        let registry_codec = Nbt::NamelessCompound {
            value: grouped
                .iter()
                .map(|(registry_id, entries)| {
                    let mut id = 0;
                    Nbt::Compound {
                        name: Some(registry_id.to_string()),
                        value: vec![
                            Nbt::String {
                                name: Some(String::from("type")),
                                value: registry_id.to_string(),
                            },
                            Nbt::List {
                                name: Some(String::from("value")),
                                value: entries
                                    .iter()
                                    .map(|e| {
                                        let n = Nbt::NamelessCompound {
                                            value: vec![
                                                Nbt::String {
                                                    name: Some("name".to_string()),
                                                    value: e.entry_id.to_string(),
                                                },
                                                Nbt::Int {
                                                    name: Some("id".to_string()),
                                                    value: id,
                                                },
                                                e.nbt
                                                    .clone()
                                                    .unwrap()
                                                    .to_named_compound("element".to_string()),
                                            ],
                                        };
                                        id = id + 1;
                                        n
                                    })
                                    .collect(),
                                tag_type: 10,
                            },
                        ],
                    }
                })
                .collect::<Vec<_>>(),
        };
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
