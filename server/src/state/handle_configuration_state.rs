use crate::client::ClientReadError;
use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_information_packet::ClientInformationPacket;
use crate::packets::configuration::plugin_message_packet::PluginMessagePacket;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};
use tracing::debug;

pub enum ConfigurationResult {
    Brand(String),
    ClientInformation,
    Play,
}

pub fn handle_configuration_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<ConfigurationResult, Box<dyn std::error::Error>> {
    match packet_id {
        PluginMessagePacket::PACKET_ID => {
            let packet = PluginMessagePacket::decode(payload)?;
            debug!("received packet {packet:?}");
            Ok(ConfigurationResult::Brand(packet.channel.to_string()))
        }
        ClientInformationPacket::PACKET_ID => {
            let packet = ClientInformationPacket::decode(payload)?;
            debug!("received packet {packet:?}");
            Ok(ConfigurationResult::ClientInformation)
        }
        AcknowledgeConfigurationPacket::PACKET_ID => {
            let packet = AcknowledgeConfigurationPacket::decode(payload)?;
            debug!("received packet {packet:?}");
            Ok(ConfigurationResult::Play)
        }
        _ => Err(Box::new(ClientReadError::UnknownPacket(
            State::Configuration,
            packet_id,
        ))),
    }
}
