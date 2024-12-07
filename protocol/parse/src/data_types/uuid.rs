use uuid::Uuid;

pub fn read_uuid(bytes: &[u8], index: &mut usize) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut data = [0u8; 16];
    data.copy_from_slice(&bytes[*index..*index + 16]);
    *index += 16;
    Ok(Uuid::from_bytes(data))
}
