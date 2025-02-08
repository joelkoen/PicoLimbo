use protocol::prelude::*;

#[derive(Debug, PacketOut)]
#[packet_id("play/clientbound/minecraft:login")]
pub struct LoginPacket {
    /// The player's Entity ID (EID).
    pub entity_id: i32,
    pub is_hardcore: bool,
    /// Size of the following array.
    /// Identifiers for all dimensions on the server.
    pub dimension_names: LengthPaddedVec<Identifier>,
    /// Was once used by the client to draw the player list, but now is ignored.
    pub max_players: VarInt,
    /// Render distance (2-32).
    pub view_distance: VarInt,
    /// The distance that the client will process specific things, such as entities.
    pub simulation_distance: VarInt,
    /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
    pub reduced_debug_info: bool,
    /// Set to false when the doImmediateRespawn gamerule is true.
    pub enable_respawn_screen: bool,
    /// Whether players can only craft recipes they have already unlocked. Currently unused by the client.
    pub do_limited_crafting: bool,
    /// The ID of the type of dimension in the minecraft:dimension_type registry, defined by the Registry Data packet.
    /// 0: overworld, 1: overworld_caves, 2: the_end, 3: the_nether
    pub dimension_type: VarInt,
    /// Name of the dimension being spawned into.
    pub dimension_name: Identifier,
    /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
    pub hashed_seed: i64,
    /// 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
    pub game_mode: u8,
    /// -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed)
    pub previous_game_mode: i8,
    /// True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
    pub is_debug: bool,
    /// True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
    pub is_flat: bool,
    /// If true, then the next two fields are present.
    pub has_death_location: bool,
    /// Name of the dimension the player died in.
    // pub death_dimension_name: Option<Identifier>,
    /// The location that the player died at.
    // pub death_location: Option<Position>,
    /// The number of ticks until the player can use the portal again.
    // pub portal_cooldown: VarInt,
    pub unknown_a: VarInt,
    pub unknown_b: VarInt,
    pub enforces_secure_chat: bool,
}

impl Default for LoginPacket {
    fn default() -> Self {
        LoginPacket {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: Vec::new().into(),
            max_players: VarInt::new(1),
            view_distance: VarInt::new(10),
            simulation_distance: VarInt::new(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::new(0),
            dimension_name: Identifier::minecraft("overworld"),
            hashed_seed: 0,
            game_mode: 3,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            // death_dimension_name: None,
            // death_location: None,
            // portal_cooldown: VarInt::default(),
            unknown_a: VarInt::default(),
            unknown_b: VarInt::default(),
            enforces_secure_chat: true,
        }
    }
}
