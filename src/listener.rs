use crate::{Connection, Handler, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

pub struct Listener {
    limit_connections: Arc<Semaphore>,
    listener: TcpListener,
}

impl Listener {
    pub fn new(listener: TcpListener, limit_connections: Arc<Semaphore>) -> Listener {
        Listener {
            listener,
            limit_connections,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.limit_connections.acquire().await.forget();

            let socket = self.accept().await?;
            let connection = Connection::new(socket);
            let limit_connections = self.limit_connections.clone();

            let mut handler = Handler::new(connection, limit_connections);

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    println!("{}", err);
                }
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
