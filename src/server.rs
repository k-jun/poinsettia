use crate::{Listener, Result, MAX_CONNECTIONS};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal::unix::Signal;
use tokio::sync::{broadcast, mpsc, Semaphore};

pub async fn run(listener: TcpListener, shutdown: Signal) -> Result<()> {
    let (notify_shutdown, _) = broadcast::channel::<()>(1);
    // let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel::<usize>(1);
    let limit_connections = Arc::new(Semaphore::new(MAX_CONNECTIONS));

    let mut server = Listener::new(listener, notify_shutdown, limit_connections);
    server.run().await
}
