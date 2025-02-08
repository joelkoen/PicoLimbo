use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use anyhow::Context;
use serde_json::Value;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tracing::debug;

#[derive(Clone)]
pub struct PacketMap {
    root_directory: PathBuf,
}

#[derive(Clone)]
pub enum PacketRecipient {
    Server,
    Client,
}

impl Display for PacketRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketRecipient::Server => f.write_str("serverbound"),
            PacketRecipient::Client => f.write_str("clientbound"),
        }
    }
}

impl PacketMap {
    pub fn new(root_directory: PathBuf) -> Self {
        Self { root_directory }
    }

    pub fn get_packet_name(
        &self,
        protocol_version: ProtocolVersion,
        state: State,
        recipient: PacketRecipient,
        packet_id: u8,
    ) -> anyhow::Result<Option<String>> {
        let json = self.get_json(protocol_version)?;

        let object = json
            .get(state.to_string())
            .unwrap()
            .get(recipient.to_string())
            .unwrap()
            .as_object()
            .unwrap();

        let packet_name = object.iter().find_map(|(key, value)| {
            if value
                .get("protocol_id")
                .and_then(|value| value.as_u64())
                .is_some_and(|protocol_id| protocol_id as u8 == packet_id)
            {
                let path = format!(
                    "{}/{}/{}",
                    state.to_string(),
                    recipient.to_string(),
                    key.to_string()
                );
                Some(path)
            } else {
                None
            }
        });

        Ok(packet_name)
    }

    pub fn get_packet_id(
        &self,
        protocol_version: ProtocolVersion,
        packet_name: &'static str,
    ) -> anyhow::Result<u8> {
        let mut json = self.get_json(protocol_version)?;

        // Traverse the JSON structure using the path provided in packet_name.
        for key in packet_name.split('/') {
            json = json
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("Key '{}' not found in JSON", key))?
                .clone();
        }

        // Extract the protocol_id value and ensure it's a valid unsigned integer.
        let protocol_id = json
            .get("protocol_id")
            .ok_or_else(|| {
                anyhow::anyhow!("Missing 'protocol_id' in JSON for packet {}", packet_name)
            })?
            .as_u64()
            .ok_or_else(|| {
                anyhow::anyhow!("Invalid 'protocol_id' type for packet {}", packet_name)
            })?;

        // Convert protocol_id to u8, ensuring it's within range.
        u8::try_from(protocol_id)
            .with_context(|| format!("protocol_id {} out of u8 range", protocol_id))
    }

    fn get_json(&self, protocol_version: ProtocolVersion) -> anyhow::Result<Value> {
        // Build the path to the JSON file.
        let packets_file_path = self
            .root_directory
            .join(protocol_version.to_string())
            .join("reports/packets.json");

        // Open the file and report an error if it fails.
        let file = File::open(&packets_file_path)
            .with_context(|| format!("Failed to open packets file: {:?}", packets_file_path))?;
        let reader = BufReader::new(file);

        // Parse the JSON file.
        serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse JSON in {:?}", packets_file_path))
    }
}

#[cfg(test)]
mod tests {
    use crate::server::packet_map::{PacketMap, PacketRecipient};
    use crate::server::protocol_version::ProtocolVersion;
    use crate::state::State;
    use std::path::PathBuf;

    #[test]
    fn get_packet_id() {
        // Given
        let packet_map = build_packet_map();
        let protocol_version = ProtocolVersion::V1_21_4;
        let packet_name = "handshake/serverbound/minecraft:intention";

        // When
        let packet_id = packet_map
            .get_packet_id(protocol_version, packet_name)
            .unwrap();

        // Then
        assert_eq!(packet_id, 0);
    }

    #[test]
    fn get_unknown_packet_id() {
        // Given
        let packet_map = build_packet_map();
        let protocol_version = ProtocolVersion::V1_21_4;
        let packet_name = "handshake/serverbound/minecraft:foo";

        // When
        let packet_id = packet_map.get_packet_id(protocol_version, packet_name);

        // Then
        assert!(packet_id.is_err());
    }

    #[test]
    fn get_packet_name() {
        // Given
        let packet_map = build_packet_map();
        let protocol_version = ProtocolVersion::V1_21_4;
        let status = State::Handshake;
        let recipient = PacketRecipient::Server;
        let packet_id = 0;

        // When
        let packet_name =
            packet_map.get_packet_name(protocol_version, status, recipient, packet_id);

        // Then
        assert_eq!(
            packet_name.unwrap().unwrap(),
            "handshake/serverbound/minecraft:intention".to_string()
        );
    }

    fn build_packet_map() -> PacketMap {
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data/generated");
        PacketMap::new(data_dir)
    }
}
