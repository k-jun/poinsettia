mod connection;
pub use connection::Connection;
mod listener;
pub use listener::Listener;

mod handler;
pub use handler::Handler;

mod command;
pub use command::Command;

mod db;
pub use db::DB;

mod client;
pub use client::Client;

pub mod server;

pub const MAX_CONNECTIONS: usize = 100;
pub const BUFFER_SIZE: usize = 4 * 1024;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
