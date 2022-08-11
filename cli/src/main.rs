mod activate;
mod cancel_process_instance;
mod create;
mod deploy;
mod fail_job;
mod publish;
mod retries;
mod set_variables;
mod status;
mod throw_error;

use std::fmt::Debug;

use async_trait::async_trait;
use clap::{AppSettings, Args, Parser, Subcommand};
use color_eyre::eyre::Result;

use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::{
    api::{
        gateway_client::GatewayClient, ResolveIncidentRequest, ResolveIncidentResponse,
    },
    Protocol, ZeebeClient,
};

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    #[clap(flatten)]
    connection: Connection,
    #[clap(subcommand)]
    command: Commands,
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
    ResolveIncident(IncidentArgs),
    CancelProcessInstance(cancel_process_instance::CancelProcessInstanceArgs),
    FailJob(fail_job::FailJobArgs),
    Create(create::CreateArgs),
    Publish(publish::PublishArgs),
    UpdateRetries(retries::UpdateRetriesArgs),
    SetVariables(set_variables::SetVariablesArgs),
    Activate(activate::ActivateArgs),
    ThrowError(throw_error::ThrowErrorArgs),
}

#[derive(Args)]
struct IncidentArgs {
    incident_key: i64,
}

#[async_trait]
impl ExecuteZeebeCommand for IncidentArgs {
    type Output = ResolveIncidentResponse;

    async fn execute(
        self,
        client: &mut ZeebeClient,
    ) -> Result<Self::Output>
    {
        Ok(client
            .resolve_incident(ResolveIncidentRequest {
                incident_key: self.incident_key,
            })
            .await?
            .into_inner())
    }
}

impl From<Connection> for zeebe_client::Connection {
    fn from(conn: Connection) -> Self {
        match (conn.address, conn.insecure) {
            (Some(addr), _) => zeebe_client::Connection::Address(addr),
            (None, true) => {
                zeebe_client::Connection::HostPort(Protocol::HTTP, conn.host, conn.port)
            }
            (None, false) => {
                zeebe_client::Connection::HostPort(Protocol::HTTPS, conn.host, conn.port)
            }
        }
    }
}

#[async_trait]
trait ExecuteZeebeCommand {
    type Output: Debug;
    async fn execute(
        self,
        client: &mut ZeebeClient,
    ) -> Result<Self::Output>;
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    let cli: Cli = Cli::parse();
    let mut client = zeebe_client::connect(cli.connection.into()).await?;
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
