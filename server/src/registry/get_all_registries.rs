use protocol::prelude::Nbt;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

const AVAILABLE_REGISTRIES: [&str; 9] = [
    "minecraft:banner_pattern",
    "minecraft:chat_type",
    "minecraft:damage_type",
    "minecraft:dimension_type",
    "minecraft:painting_variant",
    "minecraft:trim_material",
    "minecraft:trim_pattern",
    "minecraft:wolf_variant",
    "minecraft:worldgen/biome",
];

pub fn get_all_registries(root_directory: &Path) -> Vec<RegistryEntry> {
    WalkDir::new(root_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .flat_map(|entry| json_to_nbt(root_directory, entry.path()))
        .filter(|entry| AVAILABLE_REGISTRIES.contains(&entry.registry_id.as_str()))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct RegistryEntry {
    pub registry_id: String,
    pub entry_id: String,
    pub nbt: Nbt,
}

fn json_to_nbt(
    root_directory: &Path,
    path: &Path,
) -> Result<RegistryEntry, Box<dyn std::error::Error>> {
    if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
        return Err(format!("{:?} not a json file", path).into());
    }

    let registry_id = get_registry_id(root_directory, path)?;
    let entry_id = get_entry_id(path)?;
    let json = get_json_files(path)?;
    let nbt = Nbt::from_json(&json, None).to_nameless_compound();

    Ok(RegistryEntry {
        registry_id,
        entry_id,
        nbt,
    })
}

fn get_registry_id(
    root_directory: &Path,
    path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let suffix = path.strip_prefix(root_directory)?;
    Ok(format!(
        "minecraft:{}",
        suffix
            .parent()
            .ok_or("failed to read suffix")?
            .to_string_lossy()
    ))
}

fn get_entry_id(path: &Path) -> Result<String, &str> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or("Failed to get file stem")
}

fn get_json_files(path: &Path) -> std::io::Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
