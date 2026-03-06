use crate::cli::subcommand::LimaeSubCommand;
use crate::constants::{APP_NAME, CLI_VERSION};
use clap::Command;
use colored::Colorize;

#[path = "../constants.rs"]
pub mod constants;
mod subcommand;

pub fn start_cli() {
    println!(
        "{}",
        format!("{APP_NAME} v{CLI_VERSION}")
            .bold()
            .green()
            .underline()
    );

    let subcommands = Command::new(APP_NAME)
        .version(CLI_VERSION)
        .subcommand(LimaeSubCommand::Serve.build())
        .subcommand(LimaeSubCommand::Rephrase.build())
        .subcommand(LimaeSubCommand::List.build())
        .subcommand(LimaeSubCommand::Downloads.build())
        .get_matches();

    match subcommands.subcommand() {
        None => default_route(),
        Some((subcommand_name, subcommand)) => match LimaeSubCommand::get(subcommand_name) {
            LimaeSubCommand::Serve => {
                let port_number = subcommand.get_one::<String>("port").unwrap();
                println!("{}", port_number);
            }
            LimaeSubCommand::Rephrase => {
                let text = subcommand.get_one::<String>("text").unwrap();
                println!("{}", text);
            }
            LimaeSubCommand::Downloads => {}
            LimaeSubCommand::List => {}
        },
    }
}

fn default_route() {
    println!("What do you want to do?\n1. Serve\n2. Use\n3. Download");
    let mut chosen_option = String::new();
    std::io::stdin()
        .read_line(&mut chosen_option)
        .expect("failed to read the input.");

    match chosen_option.trim().parse::<u8>() {
        Ok(parsed_value) => {
            println!("chosen value: {}", parsed_value)
        }
        Err(parsed_value) => {
            eprintln!("{}", parsed_value)
        }
    };
}
