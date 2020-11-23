use crate::{Connection, Result};

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect(address: String) -> Result<Client> {
        let connection = Connection::connect(address).await?;
        Ok(Client { connection })
    }

    pub async fn get(&mut self, key: String) -> Result<String> {
        let command = format!("get {}", key);
        self.connection.write(command).await?;
        let response = self.connection.read().await?;
        Ok(response)
    }

    pub async fn set(&mut self, key: String, value: String) -> Result<String> {
        let command = format!("set {} {}", key, value);
        self.connection.write(command).await?;

        let response = self.connection.read().await?;
        Ok(response)
    }
}
