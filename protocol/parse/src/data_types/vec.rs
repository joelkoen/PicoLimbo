use crate::deserialize_packet::DeserializePacketData;
use crate::prelude::{SerializePacketData, VarInt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeVecError<T: DeserializePacketData> {
    #[error("vec length is invalid")]
    InvalidVecLength,
    #[error("error while decoding a value from the vec; error={0}")]
    DecodeError(T::Error),
}

impl<T: DeserializePacketData + std::fmt::Debug> DeserializePacketData for Vec<T> {
    type Error = DecodeVecError<T>;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        // FIXME: The length is not included in the bytes
        let length = VarInt::decode(bytes, index)
            .map_err(|_| DecodeVecError::InvalidVecLength)?
            .value();

        let mut vec = Vec::with_capacity(length as usize);

        for _ in 0..length {
            vec.push(T::decode(bytes, index).map_err(DecodeVecError::DecodeError)?);
        }

        Ok(vec)
    }
}

#[derive(Error, Debug)]
#[error("invalid vec error")]
pub enum EncodeVecError {
    EncodeError,
}

impl<T: SerializePacketData> SerializePacketData for Vec<T> {
    type Error = EncodeVecError;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        // FIXME: The length is not included in the bytes
        VarInt::new(self.len() as i32)
            .encode(bytes)
            .map_err(|_| EncodeVecError::EncodeError)?;
        for value in self {
            value
                .encode(bytes)
                .map_err(|_| EncodeVecError::EncodeError)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data_types::var_int::VarInt;
    use crate::deserialize_packet::DeserializePacketData;
    use crate::prelude::SerializePacketData;

    #[test]
    fn test_vec_decode() {
        let bytes = vec![0x02, 0x01, 0x02];
        let mut index = 0;
        let vec = Vec::<VarInt>::decode(&bytes, &mut index).unwrap();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].value(), 1);
        assert_eq!(vec[1].value(), 2);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_vec_encode() {
        let vec = vec![VarInt::new(1), VarInt::new(2)];
        let mut bytes = Vec::new();
        vec.encode(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x02, 0x01, 0x02]);
    }

    #[test]
    fn test_vec_encode_empty() {
        let vec = Vec::<VarInt>::new();
        let mut bytes = Vec::new();
        vec.encode(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x00]);
    }
}
