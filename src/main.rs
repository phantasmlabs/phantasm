use clap::{arg, ArgMatches, Command};

const START_COMMAND: &str = "start";

#[tokio::main]
async fn main() {
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
    let receiver_port = args.get_one::<u16>("receiver-port").unwrap();
    let coordinator_port = args.get_one::<u16>("coordinator-port").unwrap();
}
