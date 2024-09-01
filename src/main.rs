mod client;
mod requests;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate bon;

use clap::Parser;
use requests::*;

#[derive(Parser)]
struct Args {
    #[clap(long, env = "LINEAR_API_KEY")]
    api_key: String,

    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    Me,
    List(List),
}

#[derive(Parser)]
struct List {
    #[clap(short, long, default_value = "10")]
    n: Option<usize>,
}

#[tokio::main]
async fn main() {
    color_eyre::install().expect("color_eyre init");
    tracing_subscriber::fmt::init();

    run(Args::parse()).await.unwrap();
}

async fn run(args: Args) -> color_eyre::Result<()> {
    let client = client::Client::new(args.api_key);

    match args.subcmd {
        Cmd::Me => {
            me::print(me::request(&client).await);
        }

        Cmd::List(List { n }) => {
            list_issues::print(list_issues::request().client(&client).maybe_n(n).call().await);
        }
    }

    Ok(())
}
