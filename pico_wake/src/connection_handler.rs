use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[async_trait]
pub trait ConnectionHandler {
    async fn on_accept(&self, tcp_stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()>;

    async fn on_stop(&self) -> anyhow::Result<()>;
}
