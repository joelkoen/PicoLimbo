use protocol::prelude::*;

#[derive(Debug, PacketIn)]
#[packet_id(0x00, "configuration/serverbound/minecraft:client_information")]
pub struct ServerBoundInformationPacket {
    locale: String,
    view_distance: i8,
    chat_mode: VarInt,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listings: bool,
}
