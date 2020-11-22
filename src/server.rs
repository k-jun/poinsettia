use crate::{Listener, Result, MAX_CONNECTIONS};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

pub async fn run(listener: TcpListener) -> Result<()> {
    let limit_connections = Arc::new(Semaphore::new(MAX_CONNECTIONS));

    let mut server = Listener::new(listener, limit_connections);
    server.run().await
}
