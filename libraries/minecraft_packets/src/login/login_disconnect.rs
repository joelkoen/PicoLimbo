use minecraft_protocol::prelude::*;
use serde::Serialize;

#[derive(Debug, PacketOut)]
#[packet_id("login/clientbound/minecraft:login_disconnect")]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl Default for LoginDisconnectPacket {
    fn default() -> Self {
        Self {
            reason: r#"{"text":"Disconnected"}"#.to_owned(),
        }
    }
}

impl LoginDisconnectPacket {
    pub fn text(text: impl ToString) -> serde_json::Result<LoginDisconnectPacket> {
        let component = TextComponent::new(text);
        Ok(Self {
            reason: serde_json::to_string(&component)?,
        })
    }
}

#[derive(Serialize)]
pub struct TextComponent {
    text: String,
}

impl TextComponent {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}
