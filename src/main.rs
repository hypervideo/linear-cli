mod client;
mod requests;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate bon;

use std::path::PathBuf;

use clap::Parser;
use eyre::{Context as _, ContextCompat as _, Result};
use requests::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Parser)]
struct Args {
    #[clap(long, env = "LINEAR_API_KEY")]
    api_key: Option<String>,

    #[clap(subcommand)]
    subcmd: Cmd,
}

impl Args {
    fn json_enabled(&self) -> bool {
        match &self.subcmd {
            Cmd::Me(Me { json }) => *json,
            Cmd::List(List { json, .. }) => *json,
            Cmd::Init => false,
        }
    }
}

#[derive(Parser)]
enum Cmd {
    Me(Me),
    List(List),
    Init,
}

/// Show information about the authenticated user.
#[derive(Parser)]
struct Me {
    #[clap(long, action, default_value = "false")]
    json: bool,
}

/// List issues.
#[derive(Parser)]
struct List {
    #[clap(short, long = "limit")]
    n: Option<usize>,

    #[clap(short, long, default_value = "created")]
    sort_by: list_issues::SortBy,

    #[clap(short, long)]
    assignee: Option<String>,

    #[clap(long, value_delimiter = ',')]
    state: Option<Vec<list_issues::IssueState>>,

    #[clap(long, value_delimiter = ',')]
    not_state: Option<Vec<list_issues::IssueState>>,

    #[clap(long, action, default_value = "false")]
    json: bool,

    #[clap(long, action, default_value = "false")]
    full_width: bool,
}

#[derive(Default, Deserialize, Serialize)]
struct Config {
    api_key: String,
}

impl Config {
    fn config_file() -> Result<PathBuf> {
        let dir = directories::ProjectDirs::from("app", "linear", "linear-cli")
            .context("could not determine project directories")?;
        let config_dir = dir.config_dir();
        let config_file = config_dir.join("config.toml");
        Ok(config_file)
    }

    fn load() -> Result<Option<Self>> {
        let config_file = Self::config_file()?;
        if !config_file.exists() {
            return Ok(None);
        }

        info!(?config_file, "loading config");

        let config = std::fs::read_to_string(config_file).context("could not read config file")?;
        let config = toml::from_str::<Self>(config.as_str()).context("could not parse config file")?;
        Ok(Some(config))
    }

    fn save(&self) -> Result<()> {
        let config_file = Self::config_file()?;
        let config = toml::to_string_pretty(self).context("could not serialize config")?;
        std::fs::write(config_file, config).context("could not write config file")?;
        Ok(())
    }
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
    let config = Config::load()?;
    let api_key = args
        .api_key
        .clone()
        .or_else(|| config.map(|c| c.api_key))
        .context("no API key provided")?;

    let client = client::Client::new(api_key);

    match args.subcmd {
        Cmd::Me(Me { json }) => {
            me::print(me::request(&client).await, json);
        }

        Cmd::List(List {
            n,
            assignee,
            sort_by,
            state,
            not_state,
            json,
            full_width,
        }) => {
            let state = match (state, not_state) {
                (None, None) => None,
                (state @ Some(_), None) => state,
                (None, Some(not_state)) => Some(
                    list_issues::IssueState::iter()
                        .filter(|s| !not_state.contains(s))
                        .collect(),
                ),
                (Some(state), Some(not_state)) => Some(state.into_iter().filter(|s| !not_state.contains(s)).collect()),
            };

            list_issues::print(
                list_issues::request()
                    .client(&client)
                    .maybe_n(n)
                    .sort_by(sort_by)
                    .maybe_assignee(assignee)
                    .maybe_state(state)
                    .call()
                    .await,
                json,
                full_width,
            );
        }

        Cmd::Init => {
            let config_file = Config::config_file()?;
            println!("Initializing linear-cli. Will setup config file {config_file:?}",);
            let mut config = Config::load()?.unwrap_or_default();
            let api_key = rpassword::prompt_password("Please enter your API key: ")?;
            config.api_key = api_key;
            std::fs::create_dir_all(config_file.parent().unwrap())?;
            config.save()?;
            println!("API key saved. You can now use linear-cli.");
        }
    }

    Ok(())
}
