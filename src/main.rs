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

impl Args {
    fn json_enabled(&self) -> bool {
        match &self.subcmd {
            Cmd::Me(Me { json }) => *json,
            Cmd::List(List { json, .. }) => *json,
        }
    }
}

#[derive(Parser)]
enum Cmd {
    Me(Me),
    List(List),
}

#[derive(Parser)]
struct Me {
    #[clap(long, action, default_value = "false")]
    json: bool,
}

#[derive(Parser)]
struct List {
    #[clap(short, long, default_value = "10")]
    n: Option<usize>,

    #[clap(long, action, default_value = "false")]
    json: bool,
}

#[tokio::main]
async fn main() {
    color_eyre::install().expect("color_eyre init");

    let args = Args::parse();

    if !args.json_enabled() {
        tracing_subscriber::fmt::init();
    }

    run(Args::parse()).await.unwrap();
}

async fn run(args: Args) -> color_eyre::Result<()> {
    let client = client::Client::new(args.api_key);

    match args.subcmd {
        Cmd::Me(Me { json }) => {
            me::print(me::request(&client).await, json);
        }

        Cmd::List(List { n, json }) => {
            list_issues::print(list_issues::request().client(&client).maybe_n(n).call().await, json);
        }
    }

    Ok(())
}
