use poinsettia::{server, Result};
use structopt::StructOpt;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let port = opt.port;
    let host = opt.host;
    println!("listen on {}:{}", host, port);

    let shutdown = signal::ctrl_c();
    tokio::select! {
        res = server::run(format!("{}:{}", host, port)) => {
            if let Err(_) = res {
                println!("failed to accept");
            }
        }
        _ = shutdown => {
            println!("shutdown...")
        }
    }
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "poinsettia", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "mini redis server")]
struct Opt {
    #[structopt(short, long, default_value = "6379")]
    port: String,
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
}
