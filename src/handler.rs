use crate::{Command, Connection, ExecType, Result, DB};
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
            let response = match command.exec() {
                ExecType::Get => self.db.get(command.args()[1].clone()),
                ExecType::Set => {
                    let args = command.args();
                    self.db.set(args[1].clone(), args[2].clone())
                }
            };
            println!("response: {}", response);
        } else {
            println!("{}", "invalid command");
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
