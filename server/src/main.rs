mod cli;
mod get_packet_length;
mod handlers;
mod packet_reader;
mod packets;
mod payload;
mod raw_packet;
mod registry;
mod server;
mod state;

use crate::cli::Cli;
use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::server::{Server, SharedClient};
use crate::state::State;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    enable_logging(cli.debug);

    Server::new(cli.address)
        .on(State::Handshake, on_handshake)
        .on(State::Status, handlers::status::on_status_request)
        .on(State::Status, handlers::status::on_ping_request)
        .on(State::Login, handlers::login::on_login_start)
        .on(State::Login, handlers::login::on_login_acknowledged)
        .on(
            State::Configuration,
            handlers::configuration::on_plugin_message,
        )
        .on(
            State::Configuration,
            handlers::configuration::on_acknowledge_configuration,
        )
        .run()
        .await;
}

async fn on_handshake(client: SharedClient, packet: HandshakePacket) {
    if let Ok(state) = packet.get_next_state() {
        client.lock().await.update_state(state);
    }
}

fn enable_logging(verbose: u8) {
    let log_level = match verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2 => Level::TRACE,
        _ => Level::TRACE,
    };

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(log_level.into()))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
}
