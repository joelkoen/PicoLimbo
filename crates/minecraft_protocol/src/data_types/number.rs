use crate::prelude::DecodePacketField;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to decode number")]
pub struct DecodeNumberError;

macro_rules! impl_deserialize_packet_data {
    ($($t:ty),*) => {
        $(
            impl DecodePacketField for $t {
                type Error = DecodeNumberError;

                fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
                    let size = size_of::<Self>();
                    let slice = bytes.get(*index..*index + size)
                        .ok_or(DecodeNumberError)?;
                    let array: [u8; size_of::<Self>()] = slice.try_into()
                        .map_err(|_| DecodeNumberError)?;
                    *index += size;
                    Ok(Self::from_be_bytes(array))
                }
            }
        )*
    };
}

impl_deserialize_packet_data!(f32, f64, i32, i64, i8, u16, u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_f32() {
        // Given
        let expected_value: f32 = 1.23;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = f32::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<f32>());
    }

    #[test]
    fn test_decode_f64() {
        // Given
        let expected_value: f64 = 4.56;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = f64::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<f64>());
    }

    #[test]
    fn test_decode_i32() {
        // Given
        let expected_value: i32 = -123456;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = i32::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<i32>());
    }

    #[test]
    fn test_decode_i64() {
        // Given
        let expected_value: i64 = -9876543210;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = i64::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<i64>());
    }

    #[test]
    fn test_decode_i8() {
        // Given
        let expected_value: i8 = -42;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = i8::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<i8>());
    }

    #[test]
    fn test_decode_u16() {
        // Given
        let expected_value: u16 = 54321;
        let bytes = &expected_value.to_be_bytes();

        // When
        let mut index = 0;
        let decoded_value = u16::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, size_of::<u16>());
    }

    #[test]
    fn test_decode_u8() {
        // Given
        let expected_value: u8 = 200;
        let bytes = &[expected_value];

        // When
        let mut index = 0;
        let decoded_value = u8::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_f32_insufficient_bytes() {
        // f32 requires 4 bytes; provide only 2 bytes.
        let bytes: &[u8] = &[0x00, 0x01];
        let mut index = 0;
        let result = f32::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_f64_insufficient_bytes() {
        // f64 requires 8 bytes; provide only 4 bytes.
        let bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03];
        let mut index = 0;
        let result = f64::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_i32_insufficient_bytes() {
        // i32 requires 4 bytes; provide only 3 bytes.
        let bytes: &[u8] = &[0x00, 0x01, 0x02];
        let mut index = 0;
        let result = i32::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_i64_insufficient_bytes() {
        // i64 requires 8 bytes; provide only 5 bytes.
        let bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04];
        let mut index = 0;
        let result = i64::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_i8_insufficient_bytes() {
        // i8 requires 1 byte; provide an empty slice.
        let bytes: &[u8] = &[];
        let mut index = 0;
        let result = i8::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_u16_insufficient_bytes() {
        // u16 requires 2 bytes; provide only 1 byte.
        let bytes: &[u8] = &[0xFF];
        let mut index = 0;
        let result = u16::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_decode_u8_insufficient_bytes() {
        // u8 requires 1 byte; provide an empty slice.
        let bytes: &[u8] = &[];
        let mut index = 0;
        let result = u8::decode(bytes, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }
}
