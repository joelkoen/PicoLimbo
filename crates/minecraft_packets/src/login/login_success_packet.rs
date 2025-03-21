use minecraft_protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("login/clientbound/minecraft:login_finished")]
pub struct LoginSuccessPacket {
    uuid: Uuid,
    username: String,
    #[pvn(735..)]
    properties: LengthPaddedVec<Property>,
}

impl LoginSuccessPacket {
    pub fn new(uuid: Uuid, username: impl ToString) -> Self {
        Self {
            uuid,
            username: username.to_string(),
            properties: Vec::new().into(),
        }
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl EncodePacketField for Property {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        self.name.encode(bytes)?;
        self.value.encode(bytes)?;
        self.is_signed.encode(bytes)?;
        if let Some(signature) = &self.signature {
            signature.encode(bytes)?;
        }
        Ok(())
    }
}
