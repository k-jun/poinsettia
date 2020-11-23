use crate::{Result, BUFFER_SIZE, MAX_CONNECTIONS};
use std::io::ErrorKind::WouldBlock;
use tokio::net::{TcpListener, TcpStream};

pub struct Connection {
    stream: TcpStream,
    buffer: [u8; BUFFER_SIZE],
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let buffer = [0; BUFFER_SIZE];
        Connection { stream, buffer }
    }
    pub async fn connect(address: String) -> Result<Connection> {
        let stream = TcpStream::connect(address).await?;
        let buffer = [0; BUFFER_SIZE];

        Ok(Connection { stream, buffer })
    }

    pub async fn read(&mut self) -> Result<String> {
        let mut data = String::new();
        loop {
            self.stream.readable().await?;

            match self.stream.try_read(&mut self.buffer) {
                Ok(0) => break,
                Ok(_) => {
                    data += std::str::from_utf8(&self.buffer.to_vec())?.trim_matches(char::from(0));
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
                Err(ref e) if e.kind() == WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}
