use crate::packet_error::PacketError;
use crate::packet_handler::PacketHandler;
use crate::packets::configuration::acknowledge_finish_configuration_packet::AcknowledgeConfigurationPacket;
use crate::packets::configuration::client_known_packs_packet::ClientKnownPacksPacket;
use crate::packets::configuration::server_bound_information_packet::ServerBoundInformationPacket;
use crate::packets::configuration::server_bound_plugin_message_packet::ServerBoundPluginMessagePacket;
use crate::state::State;

pub enum ConfigurationResult {
    SendConfiguration,
    Play,
    Nothing,
}

pub fn handle_configuration_state(
    packet_id: u8,
    payload: &[u8],
) -> Result<ConfigurationResult, PacketError> {
    PacketHandler::new(State::Configuration)
        .on::<ServerBoundPluginMessagePacket>(|_| ConfigurationResult::SendConfiguration)
        .on::<ServerBoundInformationPacket>(|_| ConfigurationResult::Nothing)
        .on::<ClientKnownPacksPacket>(|_| ConfigurationResult::Nothing)
        .on::<AcknowledgeConfigurationPacket>(|_| ConfigurationResult::Play)
        .handle(packet_id, payload)
}
