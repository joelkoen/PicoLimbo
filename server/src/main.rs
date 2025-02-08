mod cli;
mod get_packet_length;
mod handlers;
mod network;
mod packets;
mod registry;
mod server;
mod state;

use crate::cli::Cli;
use crate::packets::handshaking::handshake_packet::HandshakePacket;
use crate::server::client::SharedClient;
use crate::server::server::Server;
use clap::Parser;
use handlers::configuration::{on_acknowledge_configuration, on_plugin_message};
use handlers::login::{on_login_acknowledged, on_login_start};
use handlers::status::{on_ping_request, on_status_request};
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    enable_logging(cli.debug);

    Server::new(cli.address)
        .on(on_handshake)
        .on(on_status_request)
        .on(on_ping_request)
        .on(on_login_start)
        .on(on_login_acknowledged)
        .on(on_plugin_message)
        .on(on_acknowledge_configuration)
        .run()
        .await;
}

async fn on_handshake(client: SharedClient, packet: HandshakePacket) {
    let mut client = client.lock().await;
    client.set_protocol(packet.get_protocol());

    if let Ok(state) = packet.get_next_state() {
        client.update_state(state);
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
