use crate::{Connection, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Semaphore};

pub struct Listener {
    limit_connections: Arc<Semaphore>,
    shutdown: broadcast::Sender<()>,
    listener: TcpListener,
}

impl Listener {
    pub fn new(
        listener: TcpListener,
        shutdown: broadcast::Sender<()>,
        limit_connections: Arc<Semaphore>,
    ) -> Listener {
        Listener {
            listener,
            shutdown,
            limit_connections,
        }
    }
    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.limit_connections.acquire().await.forget();

            let socket = self.accept().await?;
            let mut conn = Connection::new(socket);

            //     let mut handler = Handler {
            //         db: self.db.clone(),
            //         connection: Connection::new(socket),
            //         limit_connections: self.limit_connections.clone(),
            //         shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
            //         _shutdown_complete: self.shutdown_complete_tx.clone(),
            //     };

            tokio::spawn(async move {
                let data = conn.read().await.unwrap();
                println!("data:{}", data);
            });
        }
    }

    async fn accept(&mut self) -> Result<TcpStream> {
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
    }
}
