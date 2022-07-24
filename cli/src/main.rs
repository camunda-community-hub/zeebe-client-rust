use std::{fmt::Debug, path::PathBuf};

use clap::{AppSettings, Args, Parser, Subcommand};
use color_eyre::eyre::Result;

use zeebe_client::api::{DeployResourceRequest, Resource, TopologyRequest};

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
}

#[derive(Args)]
struct DeployArgs {
    #[clap(required = true, value_parser, value_name = "FILE")]
    resources: Vec<PathBuf>,
}

impl From<Connection> for zeebe_client::Connection {
    fn from(conn: Connection) -> Self {
        match conn.address {
            Some(addr) => zeebe_client::Connection::Address(addr),
            None => zeebe_client::Connection::HostPort(conn.host, conn.port),
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
                .deploy_resource(deploy_request(args)?)
                .await?
                .into_inner(),
        ),
    };

    println!("{:#?}", response);

    Ok(())
}

fn deploy_request(args: DeployArgs) -> Result<DeployResourceRequest> {
    let mut resoures = Vec::with_capacity(args.resources.len());
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
        resoures.push(resource);
    }
    Ok(DeployResourceRequest {
        resources: resoures,
    })
}
