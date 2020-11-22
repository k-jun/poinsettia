use crate::{Result, MAX_CONNECTIONS};
use std::io::ErrorKind::WouldBlock;
use tokio::net::{TcpListener, TcpStream};

pub struct Connection {
    stream: TcpStream,
    buffer: [u8; 4 * 1024],
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let buffer = [0; 4 * 1024];
        Connection { stream, buffer }
    }
    pub async fn read(&mut self) -> Result<String> {
        let mut data = String::new();
        loop {
            self.stream.readable().await?;

            match self.stream.try_read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => {
                    // println!("read {} bytes", n);
                    data += &std::str::from_utf8(&self.buffer.to_vec())?;
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(data)
    }
}
