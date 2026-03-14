use crate::cli::subcommand::LimaeSubCommand;
use crate::serve::serve;
use crate::{CLI_VERSION, PROJ_NAME};
use clap::Command;
use colored::Colorize;

mod subcommand;

pub fn start_cli() {
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
        None => default_route(),
        Some((subcommand_name, subcommand)) => match LimaeSubCommand::get(subcommand_name) {
            LimaeSubCommand::Serve => serve(),
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
            if parsed_value == 1 {
                serve();
            }
        }
        Err(parsed_value) => {
            eprintln!("{}", parsed_value)
        }
    };
}
