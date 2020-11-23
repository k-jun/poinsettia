use poinsettia::{server, Result};

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let opt = Opt::from_args();
    let port = opt.port;

    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    server::run(listener).await
}

#[derive(StructOpt, Debug)]
#[structopt(name = "poinsettia", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "mini redis server")]
struct Opt {
    #[structopt(short, long, default_value = "6379")]
    port: String,
}
