mod activate;
mod create;
mod publish;
mod retries;
mod set_variables;
mod throw_error;

use std::{fmt::Debug, path::PathBuf};

use clap::{AppSettings, Args, Parser, Subcommand};
use color_eyre::eyre::Result;

use set_variables::SetVariablesArgs;
use zeebe_client::{
    api::{
        CancelProcessInstanceRequest, DeployResourceRequest, FailJobRequest,
        ResolveIncidentRequest, Resource, SetVariablesRequest, TopologyRequest,
    },
    Protocol,
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
    Status,
    Deploy(DeployArgs),
    ResolveIncident(IncidentArgs),
    CancelProcessInstance(CancelProcessInstanceArgs),
    FailJob(FailJobArgs),
    Create(create::CreateArgs),
    Publish(publish::PublishArgs),
    UpdateRetries(retries::UpdateRetriesArgs),
    SetVariables(SetVariablesArgs),
    Activate(activate::ActivateArgs),
    ThrowError(throw_error::ThrowErrorArgs),
}

#[derive(Args)]
struct DeployArgs {
    #[clap(required = true, value_parser, value_name = "FILE")]
    resources: Vec<PathBuf>,
}

#[derive(Args)]
struct IncidentArgs {
    incident_key: i64,
}

#[derive(Args)]
struct CancelProcessInstanceArgs {
    process_instance_key: i64,
}

#[derive(Args)]
struct FailJobArgs {
    // the unique job identifier, as obtained when activating the job
    #[clap(required = true, short, long)]
    job_key: i64,
    // the amount of retries the job should have left
    #[clap(required = true, short, long)]
    retries: i32,
    // an optional message describing why the job failed
    // this is particularly useful if a job runs out of retries and an incident is raised,
    // as it this message can help explain why an incident was raised
    #[clap(required = false, short, long, default_value = "")]
    error_message: String,
    // the back off timeout for the next retry
    #[clap(required = false, short = 'b', long, default_value = "0")]
    retry_back_off: i64,
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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli: Cli = Cli::parse();
    let mut client = zeebe_client::connect(cli.connection.into()).await?;
    let response: Box<dyn Debug> = match cli.command {
        Commands::Status => Box::new(client.topology(TopologyRequest {}).await?.into_inner()),
        Commands::Deploy(args) => Box::new(
            client
                .deploy_resource(DeployResourceRequest::try_from(&args)?)
                .await?
                .into_inner(),
        ),
        Commands::ResolveIncident(args) => Box::new(
            client
                .resolve_incident(ResolveIncidentRequest {
                    incident_key: args.incident_key,
                })
                .await?
                .into_inner(),
        ),
        Commands::CancelProcessInstance(args) => Box::new(
            client
                .cancel_process_instance(CancelProcessInstanceRequest {
                    process_instance_key: args.process_instance_key,
                })
                .await?
                .into_inner(),
        ),
        Commands::FailJob(args) => Box::new(
            client
                .fail_job(FailJobRequest {
                    job_key: args.job_key,
                    retries: args.retries,
                    error_message: args.error_message,
                    retry_back_off: args.retry_back_off,
                })
                .await?
                .into_inner(),
        ),
        Commands::Create(args) => create::handle_create_command(&mut client, &args).await?,
        Commands::Publish(args) => publish::handle_publish_command(&mut client, &args).await?,
        Commands::UpdateRetries(args) => {
            retries::handle_set_retries_command(&mut client, &args).await?
        }

        Commands::SetVariables(arg) => Box::new(
            client
                .set_variables(SetVariablesRequest::try_from(arg)?)
                .await?
                .into_inner(),
        ),
        Commands::Activate(args) => activate::handle_activate_command(&mut client, &args).await?,
        Commands::ThrowError(args) => throw_error::handle_command(&mut client, &args).await?,
    };

    println!("{:#?}", response);

    Ok(())
}

impl TryFrom<&DeployArgs> for DeployResourceRequest {
    type Error = color_eyre::Report;

    fn try_from(args: &DeployArgs) -> Result<DeployResourceRequest, Self::Error> {
        let mut resources = Vec::with_capacity(args.resources.len());
        for path in &args.resources {
            let resource = Resource {
                name: path
                    .file_name()
                    .expect("resource path should point to a file")
                    .to_str()
                    .expect("file name should be UTF-8")
                    .to_string(),
                content: std::fs::read(path)?,
            };
            resources.push(resource);
        }
        Ok(Self { resources })
    }
}
