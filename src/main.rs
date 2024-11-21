#![warn(unused_qualifications)]

mod protos;
mod services;
mod types;

use clap::{arg, ArgMatches, Command};
use futures::{SinkExt, StreamExt};
use protos::receiver_server::ReceiverServer;
use services::{Configuration, Phantasm};
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tonic::transport::Server;
use tonic::{Request, Status};
use types::{Connection, ConnectionID};

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

fn configuration() -> Configuration {
    let auto_approve = env::var("PHANTASM_AUTO_APPROVE")
        .map(|value| value.parse::<bool>().unwrap_or_default())
        .unwrap_or_default();

    Configuration { auto_approve }
}

async fn start_handler(args: &ArgMatches) {
    // Unwrapping is safe here because the arguments are validated by Clap.
    let receiver_port = *args.get_one::<u16>("receiver-port").unwrap();
    let coordinator_port = *args.get_one::<u16>("coordinator-port").unwrap();

    let config = configuration();
    let phantasm = Phantasm::open(&config).expect("Failed to open Phantasm");
    let service = Arc::new(phantasm);

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
    let service = ReceiverServer::with_interceptor(service, auth_interceptor);
    let addr = format!("[::]:{port}").parse().unwrap();
    tracing::info!("The receiver server is ready on port {port}");

    Server::builder()
        .add_service(service)
        .serve(addr)
        .await
        .expect("Failed to start the receiver server");
}

fn auth_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    let secret = env::var("PHANTASM_SECRET").unwrap_or_default();
    if secret.is_empty() {
        return Ok(request);
    }

    let unauthorized = {
        let message = "Invalid or missing authorization key";
        Status::unauthenticated(message)
    };

    if let Some(auth) = request.metadata().get("authorization") {
        let auth = auth.to_str().map_err(|_| unauthorized.clone())?;
        if auth == secret.as_str() {
            return Ok(request);
        }
    }

    Err(unauthorized)
}

async fn start_coordinator_server(service: Arc<Phantasm>, port: u16) {
    let addr = format!("[::]:{port}");
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("The coordinator server is ready on port {port}");

    while let Ok((stream, _)) = listener.accept().await {
        let peer_addr = stream
            .peer_addr()
            .expect("Connected streams should have a peer address");

        let mut peer_ip = peer_addr.ip().to_canonical();
        if peer_ip.is_loopback() {
            peer_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        }

        tracing::info!("A connection is established with {peer_ip}");

        let service = service.clone();
        tokio::spawn(async move {
            let websocket = accept_async(stream).await.unwrap();
            let (mut writer, mut reader) = websocket.split();
            let (sender, mut receiver) = mpsc::unbounded_channel();

            let connection_id = ConnectionID::new();
            let connection = Connection::new(sender);
            service.add_connection(connection_id, connection);

            tokio::spawn(async move {
                while let Some(Ok(message)) = reader.next().await {
                    service.receive_message(&message).await
                }

                service.remove_connection(&connection_id);
                tracing::info!("A connection is closed: {peer_ip}");
            });

            tokio::spawn(async move {
                while let Some(message) = receiver.recv().await {
                    if writer.send(message).await.is_err() {
                        break;
                    }
                }
            });
        });
    }
}
