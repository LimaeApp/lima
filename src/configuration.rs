use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LimaeConfiguration {
    pub(crate) password_hash: String,
}
// hdEEuNXzpGMjCXH1v5Y0vbT4FzE57ytnEBcjm83nYxDEkok4aYYO9wiHn5uY9gYzQhgEBQ0Fhkzvi1rGdC
pub struct Password {
    pub(crate) absolute: String,
    pub(crate) hash: String,
}

pub mod configuration {
    use crate::LIMAE_CONFIG_PATH;
    use crate::configuration::{LimaeConfiguration, Password};
    use crate::utils::print_password;
    use argon2::password_hash::SaltString;
    use argon2::password_hash::rand_core::OsRng;
    use argon2::{Argon2, PasswordHasher};
    use colored::Colorize;
    use rand::distr::{Alphanumeric, SampleString};
    use rand::prelude::IteratorRandom;
    use std::fs;

    pub async fn init() {
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

    pub fn config_exists() -> bool {
        fs::exists(&*LIMAE_CONFIG_PATH).expect("Issue while locating limae config file")
    }

    pub fn write_config(limae_configuration: LimaeConfiguration) {
        let raw_config = serde_json::to_string(&limae_configuration)
            .expect("Couldn't serialize the configuration");
        fs::write(&*LIMAE_CONFIG_PATH, raw_config)
            .expect("Couldn't write the configuration to the config file");
    }

    pub fn get_a_random_password() -> Password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let random_string = format!(
            "{}",
            Alphanumeric.sample_string(
                &mut rand::rng(),
                (75..160)
                    .choose(&mut rand::rng())
                    .expect("Failed to choose random string length")
            )
        );
        let password_hash = argon2
            .hash_password(random_string.as_ref(), &salt)
            .expect("Failed to hash password")
            .to_string();

        Password {
            absolute: random_string,
            hash: password_hash,
        }
    }
}
