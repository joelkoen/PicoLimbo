use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Default, Clone, Debug)]
pub enum ProtocolVersion {
    #[default]
    V1_21_4,
    V1_21_2,
    V1_21,
    V1_20_5,
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolVersion::V1_21_4 => f.write_str("V1_21_4"),
            ProtocolVersion::V1_21_2 => f.write_str("V1_21_2"),
            ProtocolVersion::V1_21 => f.write_str("V1_21"),
            ProtocolVersion::V1_20_5 => f.write_str("V1_20_5"),
        }
    }
}

impl From<i32> for ProtocolVersion {
    fn from(value: i32) -> ProtocolVersion {
        match value {
            769 => ProtocolVersion::V1_21_4,
            768 => ProtocolVersion::V1_21_2,
            767 => ProtocolVersion::V1_21,
            766 => ProtocolVersion::V1_20_5,
            _ => ProtocolVersion::default(),
        }
    }
}

impl ProtocolVersion {
    pub fn version_number(&self) -> u32 {
        match self {
            ProtocolVersion::V1_21_4 => 769,
            ProtocolVersion::V1_21_2 => 768,
            ProtocolVersion::V1_21 => 767,
            ProtocolVersion::V1_20_5 => 766,
        }
    }
}

impl PartialEq for ProtocolVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version_number() == other.version_number()
    }
}

impl Eq for ProtocolVersion {}

impl PartialOrd for ProtocolVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProtocolVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version_number().cmp(&other.version_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version_ordering() {
        let v1_21 = ProtocolVersion::V1_21;
        let v1_21_2 = ProtocolVersion::V1_21_2;
        let v1_21_4 = ProtocolVersion::V1_21_4;

        assert!(v1_21 < v1_21_2);
        assert!(v1_21_2 < v1_21_4);
        assert!(v1_21_4 > v1_21_2);
        assert_eq!(v1_21_4, v1_21_4);
        assert_ne!(v1_21_2, v1_21_4);
    }
}
