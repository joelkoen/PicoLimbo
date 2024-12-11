use crate::packet_error::PacketError;
use crate::packets::play::client_tick_end_packet::ClientTickEndPacket;
use crate::packets::play::server_bound_keep_alive_packet::ServerBoundKeepAlivePacket;
use crate::packets::play::set_player_position_packet::{
    SetPlayerPositionAndRotationPacket, SetPlayerPositionPacket, SetPlayerRotationPacket,
};
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};

pub enum PlayResult {
    UpdatePositionAndRotation {
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
        yaw: Option<f32>,
        pitch: Option<f32>,
    },
    Nothing,
}

impl From<SetPlayerPositionAndRotationPacket> for PlayResult {
    fn from(packet: SetPlayerPositionAndRotationPacket) -> Self {
        PlayResult::UpdatePositionAndRotation {
            x: Some(packet.x),
            y: Some(packet.y),
            z: Some(packet.z),
            yaw: Some(packet.yaw),
            pitch: Some(packet.pitch),
        }
    }
}

impl From<SetPlayerRotationPacket> for PlayResult {
    fn from(packet: SetPlayerRotationPacket) -> Self {
        PlayResult::UpdatePositionAndRotation {
            x: None,
            y: None,
            z: None,
            yaw: Some(packet.yaw),
            pitch: Some(packet.pitch),
        }
    }
}

impl From<SetPlayerPositionPacket> for PlayResult {
    fn from(packet: SetPlayerPositionPacket) -> Self {
        PlayResult::UpdatePositionAndRotation {
            x: Some(packet.x),
            y: Some(packet.y),
            z: Some(packet.z),
            yaw: None,
            pitch: None,
        }
    }
}

impl From<ServerBoundKeepAlivePacket> for PlayResult {
    fn from(_: ServerBoundKeepAlivePacket) -> Self {
        PlayResult::Nothing
    }
}

impl From<ClientTickEndPacket> for PlayResult {
    fn from(_: ClientTickEndPacket) -> Self {
        PlayResult::Nothing
    }
}

pub fn handle_play_state(packet_id: u8, payload: &[u8]) -> Result<PlayResult, PacketError> {
    match packet_id {
        ServerBoundKeepAlivePacket::PACKET_ID => {
            let packet = ServerBoundKeepAlivePacket::decode(payload)?;
            Ok(packet.into())
        }
        ClientTickEndPacket::PACKET_ID => {
            let packet = ClientTickEndPacket::decode(payload)?;
            Ok(packet.into())
        }
        SetPlayerPositionAndRotationPacket::PACKET_ID => {
            let packet = SetPlayerPositionAndRotationPacket::decode(payload)?;
            Ok(packet.into())
        }
        SetPlayerRotationPacket::PACKET_ID => {
            let packet = SetPlayerRotationPacket::decode(payload)?;
            Ok(packet.into())
        }
        SetPlayerPositionPacket::PACKET_ID => {
            let packet = SetPlayerPositionPacket::decode(payload)?;
            Ok(packet.into())
        }
        _ => Err(PacketError::new(State::Play, packet_id)),
    }
}
