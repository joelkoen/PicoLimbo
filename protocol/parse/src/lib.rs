mod data_types;
mod deserialize_packet;
mod serialize_packet;
mod traits;

pub mod prelude {
    pub use crate::data_types::identifier::Identifier;
    pub use crate::data_types::optional::DecodeOptionError;
    pub use crate::data_types::position::Position;
    pub use crate::data_types::string::StringDecodingError;
    pub use crate::data_types::var_int::{VarInt, VarIntParseError};
    pub use crate::data_types::vec::DecodeVecError;
    pub use crate::deserialize_packet::DeserializePacketData;
    pub use crate::serialize_packet::SerializePacketData;
    pub use crate::traits::decode_packet::DecodePacket;
    pub use crate::traits::encode_packet::EncodePacket;
    pub use crate::traits::packet_id::PacketId;
    pub use uuid::Uuid;
}
