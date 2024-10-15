mod protos;
mod services;
mod types;

use clap::{arg, ArgMatches, Command};
use futures::{SinkExt, StreamExt};
use protos::receiver_server::ReceiverServer;
use services::Phantasm;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
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
    // Unwrapping is safe here because the arguments are validated by Clap.
    let receiver_port = *args.get_one::<u16>("receiver-port").unwrap();
    let coordinator_port = *args.get_one::<u16>("coordinator-port").unwrap();

    let service = Arc::new(Phantasm::open().expect("Failed to open Phantasm"));

    let receiver_service = service.clone();
    let receiver_server = tokio::spawn(async move {
        start_receiver_server(receiver_service, receiver_port).await
    });

    let coordinator_service = service.clone();
    let coordinator_server = tokio::spawn(async move {
        start_coordinator_server(coordinator_service, coordinator_port).await
    });

    let _ = tokio::join!(receiver_server, coordinator_server);
}

async fn start_receiver_server(service: Arc<Phantasm>, port: u16) {
    let addr = format!("[::]:{port}").parse().unwrap();
    tracing::info!("Receiver server is ready on port {port}");

    Server::builder()
        .add_service(ReceiverServer::new(service))
        .serve(addr)
        .await
        .expect("Failed to start the receiver server");
}

async fn start_coordinator_server(service: Arc<Phantasm>, port: u16) {
    let addr = format!("[::]:{port}");
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Coordinator server is ready on port {port}");

    while let Ok((stream, _)) = listener.accept().await {
        let connection_addr = stream
            .peer_addr()
            .expect("Connected streams should have a peer address");

        tracing::info!("Connection established with {connection_addr}");

        tokio::spawn(async move {
            let websocket = accept_async(stream).await.unwrap();
            let (mut writer, mut reader) = websocket.split();
            let (sender, mut receiver) = mpsc::unbounded_channel();

            tokio::spawn(async move {
                while let Some(Ok(msg)) = reader.next().await {
                    println!("Received: {}", msg.to_text().unwrap());
                }
            });

            tokio::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    if writer.send(msg).await.is_err() {
                        break;
                    }
                }
            });

            tracing::info!("Connection closed: {connection_addr}")
        });
    }
}
