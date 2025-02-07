#[derive(Clone)]
pub struct RawPacket {
    id: u8,
    data: Vec<u8>,
}

impl RawPacket {
    pub fn new(id: u8, data: &[u8]) -> Self {
        Self {
            id,
            data: data.to_vec(),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
