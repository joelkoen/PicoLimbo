#[cfg(test)]
mod tests {
    use macros::Pvn;

    #[derive(Default, Debug, Pvn, Eq, PartialEq)]
    pub enum ProtocolVersion {
        #[default]
        #[pvn(769)]
        V1_21_4,
        #[pvn(768)]
        V1_21_2,
        #[pvn(767)]
        V1_21,
    }

    #[test]
    fn test_get_large_version_number() {
        // Given
        let expected_version_number = ProtocolVersion::V1_21_4;
        let given_number = 800; // Larger than the supported pvn

        // When
        let result: ProtocolVersion = given_number.into();

        // Then
        assert_eq!(result, expected_version_number);
    }

    #[test]
    fn test_get_small_version_number() {
        // Given
        let expected_version_number = ProtocolVersion::V1_21;
        let given_number = 750; // Smaller than the supported pvn

        // When
        let result: ProtocolVersion = given_number.into();

        // Then
        assert_eq!(result, expected_version_number);
    }
}
