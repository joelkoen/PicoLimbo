use crate::network::packet_stream::{PacketStream, PacketStreamError};
use crate::network::raw_packet::RawPacket;
use crate::packets::play::client_bound_keep_alive_packet::ClientBoundKeepAlivePacket;
use crate::server::game_profile::GameProfile;
use crate::server::server::{NamedPacket, PacketMap};
use crate::state::State;
use protocol::prelude::{EncodePacket, PacketId};
use rand::Rng;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, error};

pub struct Client {
    state: State,
    packet_reader: PacketStream<TcpStream>,
    packet_map: PacketMap,
    game_profile: Option<GameProfile>,
}

impl Client {
    pub fn new(socket: TcpStream, packet_map: PacketMap) -> Self {
        let packet_reader = PacketStream::new(socket);
        Self {
            packet_reader,
            packet_map,
            state: State::default(),
            game_profile: None,
        }
    }

    pub async fn read_packet(&mut self) -> Result<Option<NamedPacket>, PacketStreamError> {
        self.packet_reader.read_packet().await.map(|packet| {
            self.get_packet_name_from_id(packet.packet_id())
                .map(|packet_name| NamedPacket {
                    name: packet_name,
                    data: packet.data().to_vec(),
                })
        })
    }

    pub fn update_state(&mut self, new_state: State) {
        debug!("update state: {}", new_state);
        self.state = new_state;
    }

    pub async fn send_packet(&mut self, packet: impl EncodePacket + PacketId) {
        let result: anyhow::Result<()> = async {
            let raw_packet = RawPacket::from_packet(packet)?;
            self.packet_reader.write_packet(raw_packet).await?;
            Ok(())
        }
        .await;

        if let Err(err) = result {
            error!("error sending packet: {:?}", err);
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub async fn send_keep_alive(&mut self) {
        // Send Keep Alive
        if self.state() == &State::Play {
            let packet = ClientBoundKeepAlivePacket::new(get_random());
            self.send_packet(packet).await;
        }
    }

    pub fn set_game_profile(&mut self, profile: GameProfile) {
        self.game_profile = Some(profile);
    }

    fn get_packet_name_from_id(&self, packet_id: u8) -> Option<String> {
        self.packet_map
            .get(&(self.state.clone(), packet_id))
            .map(|s| s.to_string())
    }
}

fn get_random() -> i64 {
    let mut rng = rand::rng();
    rng.random()
}

pub type SharedClient = Arc<Mutex<Client>>;
