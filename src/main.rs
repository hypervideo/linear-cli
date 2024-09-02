mod client;
mod requests;
mod shared;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate bon;

use std::path::PathBuf;

use clap::Parser;
use eyre::{Context as _, ContextCompat as _, Result};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Parser)]
struct Args {
    #[clap(long, env = "LINEAR_API_KEY")]
    api_key: Option<String>,

    #[clap(subcommand)]
    cmd: Command,
}

impl Args {
    fn json_enabled(&self) -> bool {
        match &self.cmd {
            Command::Init => false,
            Command::Me(Me { json }) => *json,
            Command::Issue {
                cmd: IssueCommand::List(IssueList { json, .. }),
            } => *json,
            Command::Issue {
                cmd: IssueCommand::Update(IssueUpdate { json, .. }),
            } => *json,
            Command::Team {
                cmd: TeamCommand::List(TeamList { json, .. }),
            } => *json,
            Command::Debug { .. } => false,
        }
    }
}

#[derive(Parser)]
enum Command {
    /// Initialize the configuration of `lr`. Will prompt for the API key and
    /// write $XDG_CONFIG_HOME/linear-cli/config.toml.
    Init,
    Me(Me),
    Team {
        #[clap(subcommand)]
        cmd: TeamCommand,
    },
    Issue {
        #[clap(subcommand)]
        cmd: IssueCommand,
    },

    Debug {
        #[clap(subcommand)]
        cmd: DebugCommand,
    },
}

#[derive(Parser)]
enum TeamCommand {
    List(TeamList),
}

#[derive(Parser)]
enum IssueCommand {
    List(IssueList),
    Update(IssueUpdate),
}

#[derive(Parser)]
enum DebugCommand {
    ListWorkflowStates,
}

/// Show information about the authenticated user.
#[derive(Parser)]
struct Me {
    #[clap(long, action, default_value = "false")]
    json: bool,
}

/// List teams.
#[derive(Parser)]
struct TeamList {
    #[clap(long, action, default_value = "false")]
    json: bool,

    #[clap(long, action, default_value = "false")]
    full_width: bool,
}

/// List issues.
#[derive(Parser)]
struct IssueList {
    #[clap(short, long = "limit")]
    n: Option<usize>,

    #[clap(short, long, default_value = "created")]
    sort_by: shared::SortBy,

    #[clap(short, long)]
    assignee: Option<String>,

    #[clap(long, value_delimiter = ',')]
    state: Option<Vec<shared::IssueState>>,

    #[clap(long, value_delimiter = ',')]
    not_state: Option<Vec<shared::IssueState>>,

    #[clap(long, action, default_value = "false")]
    json: bool,

    #[clap(long, action, default_value = "false")]
    full_width: bool,
}

/// List issues.
#[derive(Parser)]
struct IssueUpdate {
    #[clap(help = "Linear issue identifier (e.g. 'L-1234')")]
    id: String,

    #[clap(short, long)]
    title: Option<String>,

    #[clap(short, long)]
    state: Option<shared::IssueState>,

    #[clap(long, action, default_value = "false")]
    json: bool,
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

    match args.cmd {
        Command::Init => {
            let config_file = Config::config_file()?;
            println!("Initializing linear-cli. Will setup config file {config_file:?}",);
            let mut config = Config::load()?.unwrap_or_default();
            let api_key = rpassword::prompt_password("Please enter your API key: ")?;
            config.api_key = api_key;
            std::fs::create_dir_all(config_file.parent().unwrap())?;
            config.save()?;
            println!("API key saved. You can now use linear-cli.");
        }

        Command::Me(Me { json }) => {
            requests::me::print(requests::me::request(&client).await, json);
        }

        Command::Issue {
            cmd:
                IssueCommand::List(IssueList {
                    n,
                    assignee,
                    sort_by,
                    state,
                    not_state,
                    json,
                    full_width,
                }),
        } => {
            let state = match (state, not_state) {
                (None, None) => None,
                (state @ Some(_), None) => state,
                (None, Some(not_state)) => {
                    Some(shared::IssueState::iter().filter(|s| !not_state.contains(s)).collect())
                }
                (Some(state), Some(not_state)) => Some(state.into_iter().filter(|s| !not_state.contains(s)).collect()),
            };

            requests::issue::list::print(
                requests::issue::list::request()
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

        Command::Issue {
            cmd: IssueCommand::Update(IssueUpdate { .. }),
        } => {
            todo!()
        }

        Command::Team {
            cmd: TeamCommand::List(TeamList { json, full_width }),
        } => {
            requests::team::list::print(
                requests::team::list::request().client(&client).call().await,
                json,
                full_width,
            );
        }

        Command::Debug {
            cmd: DebugCommand::ListWorkflowStates,
        } => {
            dbg!(requests::list_workflow_states::request().client(&client).call().await?);
        }
    }

    Ok(())
}
