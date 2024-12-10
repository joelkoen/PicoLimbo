use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x02)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: LengthPaddedVec<Property>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl SerializePacketData for Property {
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
