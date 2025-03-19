use minecraft_protocol::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum PaletteContainer {
    SingleValued {
        /// Should always be 0 for Single valued palette
        bits_per_entry: u8,
        value: VarInt,
        /// Present but of length 0 when Bits Per Entry is 0.
        data: LengthPaddedVec<i64>,
    },
    Indirect {
        /// Should be 4-8 for blocks or 1-3 for biomes
        bits_per_entry: u8,
        /// Mapping of IDs in the registry to indices of this array.
        palette: LengthPaddedVec<VarInt>,
        data: LengthPaddedVec<i64>,
    },
    /// Registry IDs are stored directly as entries in the Data Array.
    Direct {
        /// Should be 15 for blocks or 6 for biomes
        bits_per_entry: u8,
        data: LengthPaddedVec<i64>,
    },
}

impl PaletteContainer {
    pub fn blocks_void() -> Self {
        Self::SingleValued {
            bits_per_entry: 0,
            value: VarInt::default(),
            data: Vec::new().into(),
        }
    }
    pub fn biomes_void() -> Self {
        Self::SingleValued {
            bits_per_entry: 0,
            value: VarInt::new(1),
            data: Vec::new().into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum PaletteContainerError {
    #[error("error while decoding a palette")]
    EncodeError,
    #[error("invalid palette container error")]
    Infallible,
}

impl From<std::convert::Infallible> for PaletteContainerError {
    fn from(_: std::convert::Infallible) -> Self {
        PaletteContainerError::Infallible
    }
}

impl From<LengthPaddedVecEncodeError> for PaletteContainerError {
    fn from(_: LengthPaddedVecEncodeError) -> Self {
        PaletteContainerError::EncodeError
    }
}

impl EncodePacketField for PaletteContainer {
    type Error = PaletteContainerError;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        match self {
            PaletteContainer::SingleValued {
                bits_per_entry,
                value,
                data,
            } => {
                bits_per_entry.encode(bytes)?;
                value.encode(bytes)?;
                data.encode(bytes)?;
            }
            PaletteContainer::Indirect {
                bits_per_entry,
                palette,
                data,
            } => {
                bits_per_entry.encode(bytes)?;
                palette.encode(bytes)?;
                data.encode(bytes)?;
            }
            PaletteContainer::Direct {
                bits_per_entry,
                data,
            } => {
                bits_per_entry.encode(bytes)?;
                data.encode(bytes)?;
            }
        }
        Ok(())
    }
}
