use crate::play::data::chunk_section::ChunkSection;
use minecraft_protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("play/clientbound/minecraft:level_chunk_with_light")]
pub struct ChunkDataAndUpdateLightPacket {
    chunk_x: i32,
    chunk_z: i32,
    // Chunk Data
    height_maps: Nbt,
    /// Size of Data in bytes!
    /// LengthPaddedVec prefixes with the number of elements!
    size: VarInt,
    data: Vec<u8>,
    block_entities: LengthPaddedVec<BlockEntity>,
    // Light Data
    sky_light_mask: BitSet,
    block_light_mask: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_light_arrays: LengthPaddedVec<Light>,
    block_light_arrays: LengthPaddedVec<Light>,
}

impl Default for ChunkDataAndUpdateLightPacket {
    fn default() -> Self {
        let long_array_tag = Nbt::LongArray {
            name: Some("MOTION_BLOCKING".to_string()),
            value: vec![0; 37],
        };
        let root_tag = Nbt::NamelessCompound {
            value: vec![long_array_tag],
        };
        let data = vec![ChunkSection::void(); 24];
        let mut encoded_data = Vec::<u8>::new();
        data.encode(&mut encoded_data).unwrap();
        let size = VarInt::new(encoded_data.len() as i32);
        Self {
            chunk_x: 0,
            chunk_z: 0,
            height_maps: root_tag,
            size,
            data: encoded_data,
            block_entities: Vec::new().into(),
            sky_light_mask: BitSet::default(),
            block_light_mask: BitSet::default(),
            empty_sky_light_mask: BitSet::default(),
            empty_block_light_mask: BitSet::default(),
            sky_light_arrays: Vec::new().into(),
            block_light_arrays: Vec::new().into(),
        }
    }
}

#[derive(Debug)]
pub struct BlockEntity {
    // TODO: Implement BlockEntity
}

impl EncodePacketField for BlockEntity {
    type Error = std::convert::Infallible;

    fn encode(&self, _bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        // Nothing to encode
        Ok(())
    }
}

#[derive(Debug)]
pub struct Light {
    /// Length of the following array is always 2048
    /// There is 1 array for each bit set to true in the light mask, starting with the lowest value. Half a byte per light value. Indexed ((y<<8) | (z<<4) | x) / 2 If there's a remainder, masked 0xF0 else 0x0F.
    block_light_array: LengthPaddedVec<i8>,
}

impl EncodePacketField for Light {
    type Error = std::convert::Infallible;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        let size = VarInt::new(self.block_light_array.0.len() as i32);
        size.encode(bytes)?;
        for &value in &self.block_light_array.0 {
            bytes.push(value as u8);
        }
        Ok(())
    }
}
