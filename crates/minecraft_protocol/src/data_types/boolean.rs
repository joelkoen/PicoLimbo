use crate::prelude::DecodePacketField;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to decode bool")]
pub struct DecodeBooleanError;

impl DecodePacketField for bool {
    type Error = DecodeBooleanError;

    fn decode(bytes: &[u8], index: &mut usize) -> Result<Self, Self::Error> {
        let value = bytes.get(*index).ok_or(DecodeBooleanError)?;
        *index += 1;
        Ok(value == &0x01)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_false_bool() {
        // Given
        let expected_value: bool = false;
        let bytes = &[0];

        // When
        let mut index = 0;
        let decoded_value = bool::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_true_bool() {
        // Given
        let expected_value: bool = true;
        let bytes = &[1];

        // When
        let mut index = 0;
        let decoded_value = bool::decode(bytes, &mut index).unwrap();

        // Then
        assert_eq!(decoded_value, expected_value);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_decode_bool_insufficient_bytes() {
        // Given
        let bytes: &[u8] = &[];
        let mut index = 0;

        // When
        let result = bool::decode(bytes, &mut index);

        // Then
        assert!(result.is_err());
        assert_eq!(index, 0);
    }
}
