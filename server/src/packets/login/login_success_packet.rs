use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x02)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub number_of_properties: VarInt,
    pub properties: Vec<Property>,
}

#[derive(Debug, PacketOut)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}
