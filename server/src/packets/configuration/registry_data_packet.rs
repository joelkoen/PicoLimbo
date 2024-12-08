use crate::packets::configuration::data::registry_entry::RegistryEntry;
use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id(0x07)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

impl RegistryDataPacket {
    pub fn dimension_type() -> Self {
        Self {
            registry_id: Identifier::minecraft("dimension_type"),
            entries: vec![RegistryEntry {
                entry_id: Identifier::minecraft("overworld"),
                has_data: true,
                nbt: Some(get_overworld_nbt()),
            }],
        }
    }

    pub fn painting_variant() -> Self {
        Self {
            registry_id: Identifier::minecraft("painting_variant"),
            entries: vec![RegistryEntry {
                entry_id: Identifier::minecraft("default"),
                has_data: true,
                nbt: Some(Nbt::NamelessCompound {
                    value: vec![
                        Nbt::String {
                            name: Some("asset_id".to_string()),
                            value: "minecraft:backyard".to_string(),
                        },
                        Nbt::Int {
                            name: Some("height".to_string()),
                            value: 3,
                        },
                        Nbt::Int {
                            name: Some("width".to_string()),
                            value: 4,
                        },
                    ],
                }),
            }],
        }
    }

    pub fn wolf_variant() -> Self {
        Self {
            registry_id: Identifier::minecraft("wolf_variant"),
            entries: vec![RegistryEntry {
                entry_id: Identifier::minecraft("default"),
                has_data: true,
                nbt: Some(Nbt::NamelessCompound {
                    value: vec![
                        Nbt::String {
                            name: Some("wild_texture".to_string()),
                            value: "minecraft:entity/wolf/wolf".to_string(),
                        },
                        Nbt::String {
                            name: Some("angry_texture".to_string()),
                            value: "minecraft:entity/wolf/wolf_angry".to_string(),
                        },
                        Nbt::String {
                            name: Some("tame_texture".to_string()),
                            value: "minecraft:entity/wolf/wolf_tame".to_string(),
                        },
                        Nbt::String {
                            name: Some("biomes".to_string()),
                            value: "minecraft:taiga".to_string(),
                        },
                    ],
                }),
            }],
        }
    }

    pub fn biome() -> Self {
        Self {
            registry_id: Identifier::minecraft("worldgen/biome"),
            entries: vec![RegistryEntry {
                entry_id: Identifier::minecraft("taiga"),
                has_data: true,
                nbt: Some(get_taiga_biome_nbt()),
            }],
        }
    }
}

fn get_taiga_biome_nbt() -> Nbt {
    Nbt::NamelessCompound {
        value: vec![
            Nbt::String {
                name: Some("precipitation".to_string()),
                value: "rain".to_string(),
            },
            Nbt::Compound {
                name: Some("effects".to_string()),
                value: vec![
                    Nbt::Int {
                        name: Some("sky_color".to_string()),
                        value: 8233983,
                    },
                    Nbt::Int {
                        name: Some("water_fog_color".to_string()),
                        value: 329011,
                    },
                    Nbt::Int {
                        name: Some("fog_color".to_string()),
                        value: 12638463,
                    },
                    Nbt::Int {
                        name: Some("water_color".to_string()),
                        value: 4159204,
                    },
                    Nbt::Compound {
                        name: Some("mood_sound".to_string()),
                        value: vec![
                            Nbt::Int {
                                name: Some("tick_delay".to_string()),
                                value: 6000,
                            },
                            Nbt::Double {
                                name: Some("offset".to_string()),
                                value: 2.0,
                            },
                            Nbt::String {
                                name: Some("sound".to_string()),
                                value: "minecraft:ambient.cave".to_string(),
                            },
                            Nbt::Int {
                                name: Some("block_search_extent".to_string()),
                                value: 8,
                            },
                        ],
                    },
                ],
            },
            Nbt::Float {
                name: Some("depth".to_string()),
                value: 0.2,
            },
            Nbt::Float {
                name: Some("temperature".to_string()),
                value: 0.25,
            },
            Nbt::Float {
                name: Some("scale".to_string()),
                value: 0.2,
            },
            Nbt::Float {
                name: Some("downfall".to_string()),
                value: 0.8,
            },
            Nbt::String {
                name: Some("category".to_string()),
                value: "taiga".to_string(),
            },
            Nbt::Byte {
                name: Some("has_precipitation".to_string()),
                value: 0,
            },
        ],
    }
}

fn get_overworld_nbt() -> Nbt {
    Nbt::NamelessCompound {
        value: vec![
            Nbt::Float {
                name: Some("ambient_light".to_string()),
                value: 0.0,
            },
            Nbt::Byte {
                name: Some("bed_works".to_string()),
                value: 1,
            },
            Nbt::Double {
                name: Some("coordinate_scale".to_string()),
                value: 1.0,
            },
            Nbt::String {
                name: Some("effects".to_string()),
                value: "minecraft:overworld".to_string(),
            },
            Nbt::Byte {
                name: Some("has_ceiling".to_string()),
                value: 0,
            },
            Nbt::Byte {
                name: Some("has_raids".to_string()),
                value: 1,
            },
            Nbt::Byte {
                name: Some("has_skylight".to_string()),
                value: 1,
            },
            Nbt::Int {
                name: Some("height".to_string()),
                value: 384,
            },
            Nbt::String {
                name: Some("infiniburn".to_string()),
                value: "#minecraft:infiniburn_overworld".to_string(),
            },
            Nbt::Int {
                name: Some("logical_height".to_string()),
                value: 384,
            },
            Nbt::Int {
                name: Some("min_y".to_string()),
                value: -64,
            },
            Nbt::Int {
                name: Some("monster_spawn_block_light_limit".to_string()),
                value: 0,
            },
            Nbt::Int {
                name: Some("monster_spawn_light_level".to_string()),
                value: 1,
            },
            Nbt::Byte {
                name: Some("natural".to_string()),
                value: 1,
            },
            Nbt::Byte {
                name: Some("piglin_safe".to_string()),
                value: 0,
            },
            Nbt::Byte {
                name: Some("respawn_anchor_works".to_string()),
                value: 0,
            },
            Nbt::Byte {
                name: Some("ultrawarm".to_string()),
                value: 0,
            },
        ],
    }
}
