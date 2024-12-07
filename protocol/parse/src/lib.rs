mod data_types;
mod deserialize_packet;
mod serialize_packet;
mod traits;
mod var_int;

pub mod prelude {
    pub use crate::deserialize_packet::DeserializePacketData;
    pub use crate::serialize_packet::SerializePacketData;
    pub use crate::traits::decode_packet::DecodePacket;
    pub use crate::traits::encode_packet::EncodePacket;
    pub use crate::var_int::{VarInt, VarIntParseError};
    pub use uuid::Uuid;
}
