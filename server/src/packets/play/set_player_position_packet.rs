use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x1C, "play/server/minecraft:move_player_pos")]
pub struct SetPlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    on_ground: bool,
}

#[derive(Debug, PacketIn)]
#[packet_id(0x1D, "play/server/minecraft:move_player_pos_rot")]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    on_ground: bool,
}

#[derive(Debug, PacketIn)]
#[packet_id(0x1E, "play/server/minecraft:move_player_rot")]
pub struct SetPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
}
