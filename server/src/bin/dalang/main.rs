use clap::{arg, Command, ArgAction, command};

#[actix_web::main]
async fn main() {
    let matches = command!()
        .subcommand(
            Command::new("start")
                .about("Start the server")
                .args(&[
                    arg!(-d --daemonize "Daemonize the process")
                        .action(ArgAction::SetTrue)
                        .group("background_start"),

                    arg!(-s --service "Create a new systemd service, enable it, and start it")
                        .action(ArgAction::SetTrue)
                        .group("background_start"),

                    arg!(-f --"static-files" <PATH> "Specify a path to serve static files, dalang serves its own by default.")
                        .action(ArgAction::Set),

                    arg!(-w --"ws-only" --"dont-serve" "Make dalang to only run its websocket server")
                        .action(ArgAction::SetTrue)
                ])
        )
        .subcommand(
            Command::new("config")
                .about("Queries and sets configuration for the server"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("config", _sub_matches)) => {
            println!("Querying and setting configuration for the app");
        }

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

        None => {
            println!("Starting the server");

            dalang_server::start(None).await
                .expect("Failed to start the server");
        }

        _ => unreachable!()
    }
}
