use poinsettia::{server, Result, Client};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();

    let address = format!("{}:{}", cli.host, cli.port);
    let mut client = Client::connect(address).await?;

    let response = match cli.command {
        Command::Get{key: k} => client.get(k).await?,
        Command::Set{key: k, value: v} => client.set(k,v).await?
    };

    println!("response: {}", response);

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "poinsettia-cli", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "mini redis server's cli")]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
    #[structopt(short, long, default_value = "6379")]
    port: String,
}

#[derive(StructOpt, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}
