use crate::play::data::palette_container::{PaletteContainer, PaletteContainerError};
use minecraft_protocol::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ChunkSection {
    /// Number of non-air blocks present in the chunk section.
    pub block_count: i16,
    /// Consists of 4096 entries, representing all the blocks in the chunk section.
    pub block_states: PaletteContainer,
    /// Consists of 64 entries, representing 4×4×4 biome regions in the chunk section.
    pub biomes: PaletteContainer,
}

impl ChunkSection {
    pub fn void() -> Self {
        Self {
            block_count: 0,
            block_states: PaletteContainer::blocks_void(),
            biomes: PaletteContainer::biomes_void(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ChunkSectionError {
    #[error("error while decoding a palette")]
    EncodeError,
    #[error("invalid palette container error")]
    Infallible,
    #[error("error while decoding a palette container")]
    PaletteContainerError,
}

impl From<std::convert::Infallible> for ChunkSectionError {
    fn from(_: std::convert::Infallible) -> Self {
        ChunkSectionError::Infallible
    }
}

impl<T: DecodePacketField> From<LengthPaddedVecDecodeError<T>> for ChunkSectionError {
    fn from(_: LengthPaddedVecDecodeError<T>) -> Self {
        ChunkSectionError::EncodeError
    }
}

impl From<PaletteContainerError> for ChunkSectionError {
    fn from(_: PaletteContainerError) -> Self {
        ChunkSectionError::PaletteContainerError
    }
}

impl EncodePacketField for ChunkSection {
    type Error = ChunkSectionError;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        self.block_count.encode(bytes)?;
        self.block_states.encode(bytes)?;
        self.biomes.encode(bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_section() {
        let chunk_section = ChunkSection::void();

        let mut buffer = Vec::new();
        chunk_section.encode(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            vec![
                /* Block count */
                0x00, 0x00,
                /* Block states */
                /* Bits Per Entry */
                0x00, /* Palette */
                /* Value */
                0x00, /* Data Array Length */
                0x00, /* Biomes */
                /* Bits Per Entry */
                0x00, /* Value */
                0x01, /* Data Array Length */
                0x00
            ]
        );
        assert_eq!(buffer.len(), 8);
    }
}
