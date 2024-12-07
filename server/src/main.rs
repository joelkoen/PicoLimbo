mod client;
mod get_packet_length;
mod packets;
mod payload;
mod state;

use crate::client::Client;
use tokio::net::TcpListener;
use tracing::{debug, error, info, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_logging(2);
    let addr = "0.0.0.0:25565";
    let listener = TcpListener::bind(&addr).await?;
    info!("listening on: {}", addr);

    while let Ok((inbound, address)) = listener.accept().await {
        debug!("accepted new client {}:{}", address.ip(), address.port());
        let mut client = Client::new(inbound, address);

        tokio::spawn(async move {
            loop {
                if let Err(err) = client.read_socket().await {
                    error!("{err}");
                    return;
                }

                // Once the payload is complete, we can break the loop to deserialize_packet the packet_in
                if client.is_complete() {
                    if let Err(err) = client
                        .handle()
                        .await
                        .and_then(|_| Ok(client.reset_payload()?))
                    {
                        error!("{err}");
                        break;
                    }
                }
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
