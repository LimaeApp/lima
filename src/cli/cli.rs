use crate::cli::subcommand::LimaeSubCommand;
use crate::configuration::configuration;
use crate::serve::serve;
use crate::{CLI_VERSION, PROJ_NAME};
use clap::Command;
use colored::Colorize;

mod subcommand;

pub async fn start_cli() {
    println!(
        "{}",
        format!("{PROJ_NAME} v{CLI_VERSION}")
            .bold()
            .green()
            .underline()
    );

    let subcommands = Command::new(PROJ_NAME)
        .version(CLI_VERSION)
        .subcommand(LimaeSubCommand::Serve.build())
        .subcommand(LimaeSubCommand::Rephrase.build())
        .subcommand(LimaeSubCommand::List.build())
        .subcommand(LimaeSubCommand::Downloads.build())
        .get_matches();

    match subcommands.subcommand() {
        None => default_route().await,
        Some((subcommand_name, subcommand)) => match LimaeSubCommand::get(subcommand_name) {
            LimaeSubCommand::Serve => serve().await,
            LimaeSubCommand::Rephrase => {
                let text = subcommand.get_one::<String>("text").unwrap();
                println!("{}", text);
            }
            LimaeSubCommand::Downloads => {}
            LimaeSubCommand::List => {}
        },
    }
}

async fn default_route() {
    println!("1. Serve\n2. Use\n3. Download\n4. Configuration");
    let mut chosen_option = String::new();
    std::io::stdin()
        .read_line(&mut chosen_option)
        .expect("failed to read the input.");

    match chosen_option.trim().parse::<u8>() {
        Ok(parsed_value) => {
            if parsed_value == 1 {
                serve().await;
            }
            if parsed_value == 4 {
                configuration::init().await
            }
        }
        Err(parsed_value) => {
            eprintln!("{}", parsed_value)
        }
    };
}
