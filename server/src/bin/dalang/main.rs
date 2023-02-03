use clap::{arg, Command, ArgAction, command};

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("start")
                .about("Start the server")
                .args(&[
                    arg!(-d --daemonize "Daemonize the process")
                        .action(ArgAction::SetTrue)
                        .group("background_start"),

                    arg!(-s --service "Create a new systemd service, enable it, and start it")
                        .action(ArgAction::SetTrue)
                        .group("background_start")
                ])
        )
        .subcommand(
            Command::new("config")
                .about("Queries and sets configuration for the server"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let daemonize = sub_matches.get_flag("daemonize");
            let service = sub_matches.get_flag("service");

            println!("Starting the server");

            if daemonize {
                println!("Daemonizing the process");
            } else if service {
                println!("Creating a new systemd service, enabling it, and starting it");
            }
        }

        Some(("config", _sub_matches)) => {
            println!("Querying and setting configuration for the app");
        }

        _ => unreachable!()
    }
}
