use crate::{Command, Connection, Result, DB};
use tokio::sync::Semaphore;

use std::sync::Arc;
pub struct Handler {
    connection: Connection,
    db: DB,
    limit_connections: Arc<Semaphore>,
}

impl Handler {
    pub fn new(connection: Connection, db: DB, limit_connections: Arc<Semaphore>) -> Handler {
        Handler {
            connection,
            db,
            limit_connections,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let raw = self.connection.read().await?;
        if let Some(command) = Command::parse(raw) {
            let response = match command {
                Command::Get { key: k } => self.db.get(k),
                Command::Set { key: k, value: v } => self.db.set(k, v),
            };
            self.connection.write(response).await?
        } else {
            let message = "invalid command".to_string();
            self.connection.write(message).await?
        }
        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        // the permit is added even if the task handling the connection panics.
        self.limit_connections.add_permits(1);
    }
}
