mod protos;
mod services;
mod types;

use clap::{arg, ArgMatches, Command};
use protos::receiver_server::ReceiverServer;
use services::Phantasm;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tonic::transport::Server;

const START_COMMAND: &str = "start";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Interface to setup and manage Phantasm servers")
        .arg_required_else_help(true)
        .subcommand(start_command())
        .get_matches();

    match cli.subcommand() {
        Some((START_COMMAND, args)) => start_handler(args).await,
        _ => unreachable!(),
    }
}

fn start_command() -> Command {
    let arg_receiver_port =
        arg!(--"receiver-port" <port> "Port for the receiver server")
            .default_value("2505")
            .value_parser(clap::value_parser!(u16));

    let arg_coordinator_port =
        arg!(--"coordinator-port" <port> "Port for the coordinator server")
            .default_value("2510")
            .value_parser(clap::value_parser!(u16));

    Command::new(START_COMMAND)
        .alias("run")
        .about("Start the receiver and the coordinator servers")
        .arg(arg_receiver_port)
        .arg(arg_coordinator_port)
}

async fn start_handler(args: &ArgMatches) {
    let receiver_port = *args.get_one::<u16>("receiver-port").unwrap();
    let coordinator_port = args.get_one::<u16>("coordinator-port").unwrap();

    let receiver_server = tokio::spawn(async move {
        supervisor(start_receiver_server, receiver_port).await
    });

    let _ = tokio::join!(receiver_server);
}

async fn start_receiver_server(port: u16) -> Result<(), Box<dyn Error>> {
    let addr = format!("[::]:{port}").parse()?;
    let service = Arc::new(Phantasm::open()?);

    tracing::info!("Receiver server is ready on port {port}");

    Server::builder()
        .add_service(ReceiverServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn supervisor<F, R>(server: F, port: u16)
where
    F: Fn(u16) -> R,
    R: Future<Output = Result<(), Box<dyn Error>>> + 'static,
{
    loop {
        if let Err(e) = server(port).await {
            tracing::error!("Server Error: {e}");
            tracing::info!("Restarting Phantasm servers...");
            sleep(Duration::from_secs(3));
        } else {
            tracing::info!("Phantasm servers exited gracefully");
            break;
        }
    }
}
