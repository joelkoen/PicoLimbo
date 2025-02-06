pub trait PacketId {
    const PACKET_ID: u8;
    const PACKET_NAME: &'static str;

    fn get_packet_id(&self) -> u8;

    fn get_packet_name(&self) -> &'static str;
}
