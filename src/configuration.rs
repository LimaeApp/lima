use crate::utils::{get_a_random_password, print_password, write_config};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LimaeConfiguration {
    pub(crate) password_hash: String,
}
// QKLrzKobWe3t4rAlSSNE55dNFR04FahJZx5Nfl5osPgybFhS3BQyAo6jjQ2P0Tv6aNdejqwz0Tqe9fvLQ5YjMQ52ctqBXjNA8Ni67BFhrtqDa1I7t7ILlRpzXQ
pub struct Password {
    pub(crate) absolute: String,
    pub(crate) hash: String,
}

pub async fn configuration() {
    println!(
        "{}",
        format!("{}\n{}", "Configuration".blue(), "1. Change Password"),
    );
    let mut chosen_option = String::new();
    std::io::stdin()
        .read_line(&mut chosen_option)
        .expect("failed to read the input.");

    match chosen_option.trim().parse::<u8>() {
        Ok(parsed_value) => {
            if parsed_value == 1 {
                let password = get_a_random_password();
                write_config(LimaeConfiguration {
                    password_hash: password.hash,
                });
                print_password(&password.absolute);
            }
        }
        Err(parsed_value) => {
            eprintln!("{}", parsed_value)
        }
    };
}
