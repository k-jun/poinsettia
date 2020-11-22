mod connection;
pub use connection::Connection;
mod listener;
pub use listener::Listener;

pub mod server;

pub const MAX_CONNECTIONS: usize = 100;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
