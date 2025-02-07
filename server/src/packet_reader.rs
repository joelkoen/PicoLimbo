use crate::payload::{Payload, PayloadAppendError};
use crate::raw_packet::RawPacket;
use protocol::prelude::{EncodePacket, PacketId, SerializePacketData, VarInt};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, trace};

pub struct PacketStream {
    socket: TcpStream,
    payload: Payload,
}

impl PacketStream {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            payload: Payload::new(),
        }
    }

    pub async fn read_packet(&mut self) -> Result<RawPacket, PacketReaderError> {
        while !self.payload.is_complete() {
            let mut buf = vec![0; self.payload.get_remaining_to_read()];

            let bytes_received = self.socket.read(&mut buf).await?;

            if bytes_received == 0 {
                self.socket.write_all(&[0]).await?;
            }

            self.payload
                .append_bytes(&buf[..bytes_received], bytes_received)?;
        }

        let bytes = self.payload.get_data();
        let packet_id = bytes[0];
        let packet_payload = &bytes[1..];
        let raw_packet = RawPacket::new(packet_id, packet_payload);
        self.payload.reset()?;

        Ok(raw_packet)
    }

    pub async fn write_packet(
        &mut self,
        packet: impl EncodePacket + PacketId,
    ) -> Result<(), PacketReaderError> {
        debug!(
            "writing packet {} (0x{:02x})",
            packet.get_packet_name(),
            packet.get_packet_id()
        );
        let encoded_packet = packet
            .encode()
            .map_err(|_| PacketReaderError::EncodeError)?;
        let mut payload = Vec::new();
        VarInt::new(encoded_packet.len() as i32 + 1)
            .encode(&mut payload)
            .map_err(|_| PacketReaderError::EncodeError)?;
        payload.push(packet.get_packet_id());
        payload.extend_from_slice(&encoded_packet);
        trace!("{}", print_bytes_hex(&payload, payload.len()));
        self.socket.write_all(&payload).await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum PacketReaderError {
    #[error("invalid packet_in received; error={0}")]
    InvalidPacket(#[from] PayloadAppendError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("encode packet error")]
    EncodeError,
}

#[allow(dead_code)]
pub fn print_bytes_hex(bytes: &[u8], length: usize) -> String {
    bytes[..length]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
