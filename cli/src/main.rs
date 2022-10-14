mod activate_jobs;
mod cancel_process_instance;
mod complete_job;
mod create_process_instance;
mod deploy_resource;
mod fail_job;
mod publish_message;
mod resolve_incident;
mod set_variables;
mod status;
mod throw_error;
mod update_retries;

use std::fmt::Debug;

use async_trait::async_trait;
use clap::{Parser, Subcommand};

use color_eyre::eyre::Result;
use zeebe_client::ZeebeClient;

#[derive(Parser)]
#[clap()]
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
    #[arg(long, group = "authentication", env = "ZEEBE_CLIENT_ID")]
    client_id: Option<String>,
    #[arg(long, env = "ZEEBE_CLIENT_SECRET")]
    client_secret: Option<String>,
    #[arg(
        long,
        env = "ZEEBE_AUTHORIZATION_SERVER_URL",
        default_value = "https://login.cloud.camunda.io/oauth/token/"
    )]
    authorization_server: String,
}

#[derive(Parser)]
#[command(group = clap::ArgGroup::new("connection"))]
struct Connection {
    #[arg(long)]
    insecure: bool,

    #[arg(long, group = "connection", env = "ZEEBE_ADDRESS")]
    address: Option<String>,

    #[arg(
        short,
        long,
        group = "connection",
        conflicts_with = "address",
        default_value = "localhost",
        env = "ZEEBE_HOST"
    )]
    host: String,
    #[arg(
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
    // status
    Status(status::StatusArgs), //aka topology

    // deployment
    DeployResource(deploy_resource::DeployResourceArgs),

    // process instance
    CreateProcessInstance(create_process_instance::CreateProcessInstanceArgs),
    CancelProcessInstance(cancel_process_instance::CancelProcessInstanceArgs),

    // message
    PublishMessage(publish_message::PublishMessageArgs),

    // incident
    ResolveIncident(resolve_incident::ResolveIncidentArgs),

    // variables
    SetVariables(set_variables::SetVariablesArgs),

    //jobs
    ActivateJobs(activate_jobs::ActivateJobsArgs),
    CompleteJob(complete_job::CompleteJobArgs),
    FailJob(fail_job::FailJobArgs),
    UpdateRetries(update_retries::UpdateRetriesArgs),
    ThrowError(throw_error::ThrowErrorArgs),
}

impl From<Connection> for zeebe_client::Connection {
    fn from(conn: Connection) -> Self {
        match conn.address {
            Some(addr) => zeebe_client::Connection {
                insecure: conn.insecure,
                addr,
            },
            None => zeebe_client::Connection {
                insecure: conn.insecure,
                addr: format!("{}:{}", conn.host, conn.port),
            },
        }
    }
}

impl Authentication {
    fn for_connection(
        &self,
        conn: &zeebe_client::Connection,
    ) -> Result<zeebe_client::Authentication> {
        match (&self.client_id, &self.client_secret) {
            (None, None) => Ok(zeebe_client::Authentication::Unauthenticated),
            (Some(client_id), Some(client_secret)) => {
                let audience = conn
                    .addr
                    .rsplit_once(':')
                    .map(|(authority, _port)| authority)
                    .unwrap_or(&conn.addr)
                    .to_owned();
                Ok(zeebe_client::Authentication::Oauth2(
                    zeebe_client::auth::OAuth2Config {
                        client_id: client_id.clone(),
                        client_secret: client_secret.clone(),
                        auth_server: self.authorization_server.clone(),
                        audience,
                    },
                ))
            }
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
    let conn: zeebe_client::Connection = cli.connection.into();
    let mut client: ZeebeClient =
        zeebe_client::connect(conn.clone(), cli.auth.for_connection(&conn)?).await?;
    let response: Box<dyn Debug> = match cli.command {
        Commands::ActivateJobs(args) => Box::new(args.execute(&mut client).await?),
        Commands::CancelProcessInstance(args) => Box::new(args.execute(&mut client).await?),
        Commands::CompleteJob(args) => Box::new(args.execute(&mut client).await?),
        Commands::CreateProcessInstance(args) => args.execute(&mut client).await?, // Already boxed, because it could be one of two results
        Commands::DeployResource(args) => Box::new(args.execute(&mut client).await?),
        Commands::FailJob(args) => Box::new(args.execute(&mut client).await?),
        Commands::PublishMessage(args) => Box::new(args.execute(&mut client).await?),
        Commands::ResolveIncident(args) => Box::new(args.execute(&mut client).await?),
        Commands::SetVariables(args) => Box::new(args.execute(&mut client).await?),
        Commands::Status(args) => Box::new(args.execute(&mut client).await?),
        Commands::ThrowError(args) => Box::new(args.execute(&mut client).await?),
        Commands::UpdateRetries(args) => Box::new(args.execute(&mut client).await?),
    };

    println!("{:#?}", response);

    Ok(())
}
