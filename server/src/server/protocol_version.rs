use std::fmt::Display;

#[derive(Default, Clone)]
pub enum ProtocolVersion {
    #[default]
    V1_21_4,
    V1_21_2,
    V1_21,
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolVersion::V1_21_4 => f.write_str("V1_21_4"),
            ProtocolVersion::V1_21_2 => f.write_str("V1_21_2"),
            ProtocolVersion::V1_21 => f.write_str("V1_21"),
        }
    }
}

impl From<i32> for ProtocolVersion {
    fn from(value: i32) -> ProtocolVersion {
        match value {
            769 => ProtocolVersion::V1_21_4,
            768 => ProtocolVersion::V1_21_2,
            767 => ProtocolVersion::V1_21,
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
        }
    }
}
