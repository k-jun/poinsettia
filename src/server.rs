use crate::{Listener, Result, DB, MAX_CONNECTIONS};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Semaphore;

pub async fn run(address: String) -> Result<()> {
    let listener = TcpListener::bind(&address).await?;
    let limit_connections = Arc::new(Semaphore::new(MAX_CONNECTIONS));
    let db = DB::new();

    let mut server = Listener::new(listener, db, limit_connections);
    server.run().await
}
