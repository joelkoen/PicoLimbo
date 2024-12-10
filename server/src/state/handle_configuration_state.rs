use crate::packet_error::PacketError;
use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_known_packs_packet::ClientKnownPacksPacket;
use crate::packets::configuration::server_bound_information_packet::ServerBoundInformationPacket;
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

pub enum ConfigurationResult {
    SendConfiguration,
    Play,
    Nothing,
}

pub fn handle_configuration_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<ConfigurationResult, PacketError> {
    match packet_id {
        ServerBoundPluginMessagePacket::PACKET_ID => {
            ServerBoundPluginMessagePacket::decode(payload)?;
            Ok(ConfigurationResult::SendConfiguration)
        }
        ServerBoundInformationPacket::PACKET_ID => {
            ServerBoundInformationPacket::decode(payload)?;
            Ok(ConfigurationResult::Nothing)
        }
        ClientKnownPacksPacket::PACKET_ID => {
            ClientKnownPacksPacket::decode(payload)?;
            Ok(ConfigurationResult::Nothing)
        }
        AcknowledgeConfigurationPacket::PACKET_ID => {
            AcknowledgeConfigurationPacket::decode(payload)?;
            Ok(ConfigurationResult::Play)
        }
        _ => Err(PacketError::new(State::Configuration, packet_id)),
    }
}
