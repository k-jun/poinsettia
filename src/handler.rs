use crate::{Command, Connection, ExecType, Result};
use tokio::sync::Semaphore;

use std::sync::Arc;
pub struct Handler {
    // db: DB,
    connection: Connection,
    limit_connections: Arc<Semaphore>,
}

impl Handler {
    pub fn new(connection: Connection, limit_connections: Arc<Semaphore>) -> Handler {
        Handler {
            connection,
            limit_connections,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // println!("{}", );
        let raw = self.connection.read().await?;
        if let Some(command) = Command::parse(raw) {
            println!("command:{:?}", command);
            match command.exec() {
                ExecType::Get => (),
                ExecType::Set => (),
            };
        } else {
            // invalid command
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
