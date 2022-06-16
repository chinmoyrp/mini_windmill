use std::net::SocketAddr;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    backend::run_server(addr).await?;

    Ok(())
}
