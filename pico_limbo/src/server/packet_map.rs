use crate::data::packets_report::packet_mapping::PacketMapping;
use crate::server::protocol_version::ProtocolVersion;
use crate::state::State;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

type PacketReportCache = HashMap<ProtocolVersion, Arc<PacketMapping>>;

#[derive(Clone)]
pub struct PacketMap {
    root_directory: PathBuf,
    cached_packet_reports: Arc<RwLock<PacketReportCache>>,
}

impl PacketMap {
    pub fn new(root_directory: PathBuf) -> Self {
        Self {
            root_directory,
            cached_packet_reports: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_packet_name(
        &self,
        protocol_version: &ProtocolVersion,
        state: &State,
        packet_id: u8,
    ) -> anyhow::Result<Option<String>> {
        let prefix = format!("{}/serverbound", state.to_string());
        Ok(self
            .get_mapping(&protocol_version)?
            .get_name(packet_id, &prefix))
    }

    pub fn get_packet_id(
        &self,
        protocol_version: &ProtocolVersion,
        packet_path: &'static str,
    ) -> anyhow::Result<Option<u8>> {
        Ok(self.get_mapping(protocol_version)?.get_id(&packet_path))
    }

    fn get_mapping(
        &self,
        protocol_version: &ProtocolVersion,
    ) -> anyhow::Result<Arc<PacketMapping>> {
        {
            // Check the cache using a read lock.
            let cache = self
                .cached_packet_reports
                .read()
                .map_err(|e| anyhow::anyhow!("Failed to acquire read lock: {}", e))?;
            if let Some(mapping) = cache.get(protocol_version) {
                return Ok(mapping.clone());
            }
        }

        // Not in cache: compute the file path and load the mapping.
        let packets_file_path = self
            .root_directory
            .join(protocol_version.to_string())
            .join("reports/packets.json");

        let report_mapping = PacketMapping::from_file(packets_file_path)?;
        let mapping_arc = Arc::new(report_mapping);

        // Insert into the cache using a write lock.
        let mut cache = self
            .cached_packet_reports
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire write lock: {}", e))?;
        let entry = cache
            .entry(protocol_version.clone())
            .or_insert_with(|| mapping_arc.clone());
        Ok(entry.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::server::packet_map::PacketMap;
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
        let packet_id = packet_map.get_packet_id(&protocol_version, packet_name);

        // Then
        assert_eq!(packet_id.unwrap().unwrap(), 0);
    }

    #[test]
    fn get_unknown_packet_id() {
        // Given
        let packet_map = build_packet_map();
        let protocol_version = ProtocolVersion::V1_21_4;
        let packet_name = "handshake/serverbound/minecraft:foo";

        // When
        let packet_id = packet_map.get_packet_id(&protocol_version, packet_name);

        // Then
        assert!(packet_id.unwrap().is_none());
    }

    #[test]
    fn get_packet_name() {
        // Given
        let packet_map = build_packet_map();
        let protocol_version = ProtocolVersion::V1_21_4;
        let status = State::Handshake;
        let packet_id = 0;

        // When
        let packet_name = packet_map.get_packet_name(&protocol_version, &status, packet_id);

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
