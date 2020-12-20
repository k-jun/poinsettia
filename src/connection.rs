use crate::{Result, BUFFER_SIZE};
use std::io::ErrorKind::WouldBlock;
use std::net::Shutdown;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let buffer = vec![0; BUFFER_SIZE];
        Connection { stream, buffer }
    }
    pub async fn connect(address: String) -> Result<Connection> {
        let stream = TcpStream::connect(address).await?;
        let buffer = vec![0; BUFFER_SIZE];

        Ok(Connection { stream, buffer })
    }

    pub async fn read(&mut self) -> Result<String> {
        let mut data = String::new();
        loop {
            self.stream.readable().await?;
            match self.stream.try_read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => {
                    self.buffer.truncate(n);
                    data += std::str::from_utf8(&self.buffer)?;
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(data)
    }

    pub async fn write(&mut self, data: String) -> Result<()> {
        let bytes = data.into_bytes();
        loop {
            self.stream.writable().await?;
            match self.stream.try_write(&bytes) {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        self.stream.shutdown(Shutdown::Write)?;
        Ok(())
    }
}
