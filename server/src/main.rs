use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tracing::{debug, error, info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use packet::ParsePacket;

#[derive(Debug, ParsePacket)]
pub struct HandshakePacket {
    pub protocol: i32,
    pub hostname: String,
    pub port: u16,
    pub next_state: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_logging(2);
    let addr = "127.0.0.0:25565";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    while let Ok((mut inbound, address)) = listener.accept().await {
        debug!("Accepted new client {}:{}", address.ip(), address.port());

        tokio::spawn(async move {
            let mut buf = vec![0; 16_384];

            let bytes_received = inbound.read(&mut buf).await;

            if let Ok(bytes_received) = bytes_received {
                debug!("Received {} bytes", bytes_received);
            } else {
                error!("Failed to read from socket");
            }
        });
    }

    Ok(())
}

pub fn enable_logging(verbose: u8) {
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
