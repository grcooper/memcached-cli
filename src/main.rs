use clap::Parser;
use memcached::Client;
use memcached::proto::ProtoType;
use dialoguer::{theme::ColorfulTheme, Input};

mod commands;

use commands::generate_command;
use commands::quit::QUIT;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Host to connect to
    #[arg(short, long)]
    server: String,
}

fn main() {
    let args = Args::parse();
    let mut tcp_string = String::from("tcp://");
    tcp_string.push_str(&args.server);

    let servers = [(&tcp_string, 1)];

    let client = &mut Client::connect(&servers, ProtoType::Binary).unwrap();

    loop {
        let command: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(">")
            .interact_text()
            .unwrap();

        let command_parts: Vec<&str> = command.split(" ").collect();

        let mut command_action: Box<dyn commands::Command> = generate_command(command_parts);
        let result = command_action.execute(client);
        match result {
            Ok(s) => {
                if s == QUIT {
                    println!("Gracefully shutting down");
                    break;
                }
                println!("{}", s);
            }
            Err(e) => {
                println!("Unexpected error: {}", e.to_string())
            }
        }
    }
}


