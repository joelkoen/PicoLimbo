use crate::client::ClientReadError;
use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_known_packs_packet::ClientKnownPacksPacket;
use crate::packets::configuration::server_bound_information_packet::ServerBoundInformationPacket;
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

pub enum ConfigurationResult {
    Brand(String),
    ClientInformation,
    Play,
    KnownPacks,
}

pub fn handle_configuration_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<ConfigurationResult, Box<dyn std::error::Error>> {
    match packet_id {
        ServerBoundPluginMessagePacket::PACKET_ID => {
            let packet = ServerBoundPluginMessagePacket::decode(payload)?;
            Ok(ConfigurationResult::Brand(packet.channel.to_string()))
        }
        ServerBoundInformationPacket::PACKET_ID => {
            ServerBoundInformationPacket::decode(payload)?;
            Ok(ConfigurationResult::ClientInformation)
        }
        ClientKnownPacksPacket::PACKET_ID => {
            ClientKnownPacksPacket::decode(payload)?;
            Ok(ConfigurationResult::KnownPacks)
        }
        AcknowledgeConfigurationPacket::PACKET_ID => {
            AcknowledgeConfigurationPacket::decode(payload)?;
            Ok(ConfigurationResult::Play)
        }
        _ => Err(Box::new(ClientReadError::UnknownPacket(
            State::Configuration,
            packet_id,
        ))),
    }
}
