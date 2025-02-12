use minecraft_packets::login::login_state_packet::LoginStartPacket;
use minecraft_protocol::prelude::Uuid;

#[derive(Debug, Clone)]
pub struct GameProfile {
    username: String,
    uuid: Uuid,
}

impl GameProfile {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl From<LoginStartPacket> for GameProfile {
    fn from(value: LoginStartPacket) -> Self {
        Self {
            username: value.name,
            uuid: value.player_uuid,
        }
    }
}
