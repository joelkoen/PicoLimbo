use anyhow::anyhow;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

pub struct PacketMapping {
    name_to_id: HashMap<String, u8>,
}

impl PacketMapping {
    pub fn get_id(&self, name: &str) -> Option<u8> {
        self.name_to_id.get(name).cloned()
    }

    pub fn get_name(&self, id: u8, prefix: &str) -> Option<String> {
        self.name_to_id
            .iter()
            .find(|(key, &protocol_id)| protocol_id == id && key.starts_with(prefix))
            .map(|(key, _)| key.clone())
    }

    /// Reads the JSON file at `path` and builds the mapping.
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Self::from_json_str(&data)
    }

    /// Builds the mapping from a JSON string.
    pub fn from_json_str(json_str: &str) -> anyhow::Result<Self> {
        let value: Value = serde_json::from_str(json_str)?;
        let mut name_to_id = HashMap::new();

        // The JSON is expected to be an object at the top level.
        let states = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected a JSON object at the top level"))?;
        for (state, state_value) in states {
            // Each state (like "handshake", "login", etc.) should be an object.
            let directions = state_value
                .as_object()
                .ok_or_else(|| anyhow!("Expected state value to be an object"))?;
            for (direction, direction_value) in directions {
                // Each direction (like "clientbound" or "serverbound") is an object.
                let packets = direction_value
                    .as_object()
                    .ok_or_else(|| anyhow!("Expected direction value to be an object"))?;
                for (packet_name, packet_value) in packets {
                    // Each packet is represented by an object containing at least "protocol_id".
                    let packet_obj = packet_value
                        .as_object()
                        .ok_or_else(|| anyhow!("Expected packet value to be an object"))?;
                    // Get the protocol_id from the packet object.
                    if let Some(protocol_id_value) = packet_obj.get("protocol_id") {
                        let protocol_id_u64 = protocol_id_value
                            .as_u64()
                            .ok_or_else(|| anyhow!("protocol_id is not a valid u64"))?;
                        if protocol_id_u64 > u8::MAX as u64 {
                            return Err(anyhow!("protocol_id is out of u8 range"));
                        }
                        // Build the composite key.
                        let key = format!("{}/{}/{}", state, direction, packet_name);
                        name_to_id.insert(key, protocol_id_u64 as u8);
                    }
                }
            }
        }
        Ok(PacketMapping { name_to_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_mapping_from_json() {
        let json_str = r#"
        {
          "handshake": {
            "serverbound": {
              "minecraft:intention": {
                "protocol_id": 0
              }
            }
          },
          "login": {
            "clientbound": {
              "minecraft:game_profile": {
                "protocol_id": 2
              }
            },
            "serverbound": {
              "minecraft:hello": {
                "protocol_id": 0
              }
            }
          }
        }
        "#;

        let mapping = PacketMapping::from_json_str(json_str).unwrap();

        // Check one of the keys.
        let key = "handshake/serverbound/minecraft:intention";
        assert_eq!(mapping.get_id(key), Some(0));

        let key2 = "login/serverbound/minecraft:hello";
        assert_eq!(mapping.get_id(key2), Some(0));

        let key3 = "login/clientbound/minecraft:game_profile";
        assert_eq!(mapping.get_id(key3), Some(2));

        let key4 = "handshake/serverbound/minecraft:intention";
        assert_eq!(
            mapping.get_name(0, "handshake/serverbound"),
            Some(key4.to_string())
        );
    }
}
