mod activate;
mod cancel_process_instance;
mod create;
mod deploy;
mod fail_job;
mod incident;
mod publish;
mod retries;
mod set_variables;
mod status;
mod throw_error;

use std::fmt::Debug;

use async_trait::async_trait;
use clap::{AppSettings, Parser, Subcommand};
use color_eyre::eyre::Report;
use color_eyre::eyre::Result;
use zeebe_client::ZeebeClient;

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    #[clap(flatten)]
    connection: Connection,
    #[clap(flatten)]
    auth: Authentication,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct Authentication {
    #[clap(long, value_parser, group = "authentication", env = "ZEEBE_CLIENT_ID")]
    client_id: Option<String>,
    #[clap(long, value_parser, env = "ZEEBE_CLIENT_SECRET")]
    client_secret: Option<String>,
    #[clap(
        long,
        value_parser,
        env = "ZEEBE_AUTHORIZATION_SERVER_URL",
        default_value = "https://login.cloud.camunda.io/oauth/token/"
    )]
    authorization_server: String,
}

#[derive(Parser)]
#[clap(group = clap::ArgGroup::new("connection"))]
struct Connection {
    #[clap(long)]
    insecure: bool,

    #[clap(long, value_parser, group = "connection", env = "ZEEBE_ADDRESS")]
    address: Option<String>,

    #[clap(
        short,
        long,
        group = "connection",
        conflicts_with = "address",
        default_value = "localhost",
        env = "ZEEBE_HOST"
    )]
    host: String,
    #[clap(
        short,
        long,
        group = "connection",
        conflicts_with = "address",
        value_parser = clap::value_parser!(u16).range(1..),
        default_value_t = 26500,
        env = "ZEEBE_PORT")]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
    Status(status::StatusArgs),
    Deploy(deploy::DeployArgs),
    ResolveIncident(incident::IncidentArgs),
    CancelProcessInstance(cancel_process_instance::CancelProcessInstanceArgs),
    FailJob(fail_job::FailJobArgs),
    Create(create::CreateArgs),
    Publish(publish::PublishArgs),
    UpdateRetries(retries::UpdateRetriesArgs),
    SetVariables(set_variables::SetVariablesArgs),
    Activate(activate::ActivateArgs),
    ThrowError(throw_error::ThrowErrorArgs),
}

impl From<Connection> for zeebe_client::Connection {
    fn from(conn: Connection) -> Self {
        match conn.address {
            Some(addr) => zeebe_client::Connection::Address {
                insecure: conn.insecure,
                addr,
            },
            None => zeebe_client::Connection::HostPort {
                insecure: conn.insecure,
                host: conn.host,
                port: conn.port,
            },
        }
    }
}

impl TryFrom<Authentication> for zeebe_client::Authentication {
    type Error = Report;

    fn try_from(auth: Authentication) -> Result<zeebe_client::Authentication> {
        match (auth.client_id, auth.client_secret) {
            (None, None) => Ok(zeebe_client::Authentication::Unauthenticated),
            (Some(client_id), Some(client_secret)) => Ok(zeebe_client::Authentication::Oauth2(
                zeebe_client::auth::OAuth2Config {
                    client_id,
                    client_secret,
                    auth_server: auth.authorization_server,
                    audience: "zeebe".to_owned(),
                },
            )),
            _ => Err(color_eyre::eyre::eyre!(
                "Partial authentication info, needs at least client id and client secret"
            )),
        }
    }
}

#[async_trait]
trait ExecuteZeebeCommand {
    type Output: Debug;
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output>;
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;
    use tracing_tree::HierarchicalLayer;

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(HierarchicalLayer::new(2))
        .with(ErrorLayer::default())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    install_tracing();

    color_eyre::install()?;

    let cli: Cli = Cli::parse();
    let mut client: ZeebeClient =
        zeebe_client::connect(cli.connection.into(), cli.auth.try_into()?).await?;
    let response: Box<dyn Debug> = match cli.command {
        Commands::Status(args) => Box::new(args.execute(&mut client).await?),
        Commands::Deploy(args) => Box::new(args.execute(&mut client).await?),
        Commands::ResolveIncident(args) => Box::new(args.execute(&mut client).await?),
        Commands::CancelProcessInstance(args) => Box::new(args.execute(&mut client).await?),
        Commands::FailJob(args) => Box::new(args.execute(&mut client).await?),
        Commands::Create(args) => args.execute(&mut client).await?, // Already boxed
        Commands::Publish(args) => args.execute(&mut client).await?, // Already boxed,
        Commands::UpdateRetries(args) => Box::new(args.execute(&mut client).await?),
        Commands::SetVariables(args) => Box::new(args.execute(&mut client).await?),
        Commands::Activate(args) => args.execute(&mut client).await?, // Already boxed
        Commands::ThrowError(args) => Box::new(args.execute(&mut client).await?),
    };

    println!("{:#?}", response);

    Ok(())
}
