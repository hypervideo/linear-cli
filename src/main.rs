mod client;
mod requests;

#[macro_use]
extern crate tracing;

use clap::Parser;

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
    // #[clap(long, env = "LINEAR_TEAM_ID")]
    // team_id: String,
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
            let me = requests::me(&client).await?;
            println!("{}", serde_json::to_string_pretty(&me)?);
        }

        Cmd::List(_) => {
            let issues = requests::list_issues(&client).await?;
            for issue in issues {
                println!("{}", serde_json::to_string_pretty(&issue)?);
            }
        }
    }

    Ok(())
}
