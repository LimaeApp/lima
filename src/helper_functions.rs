use crate::configuration::{LimaeConfiguration, Password};
use crate::{LIMAE_CONFIG, LIMAE_CONFIG_PATH};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;
use colored::Colorize;
use rand::distr::{Alphanumeric, SampleString};
use rand::prelude::IteratorRandom;

pub fn write_config(limae_configuration: LimaeConfiguration) {
    let raw_config =
        serde_json::to_string(&limae_configuration).expect("Couldn't serialize the configuration");
    std::fs::write(&*LIMAE_CONFIG_PATH, raw_config)
        .expect("Couldn't write the configuration to the config file");
}

pub fn get_a_random_password() -> Password {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let random_string = format!(
        "{}",
        Alphanumeric.sample_string(
            &mut rand::rng(),
            (75..160).choose(&mut rand::rng()).unwrap()
        )
    );
    let password_hash = argon2
        .hash_password(random_string.as_ref(), &salt)
        .unwrap()
        .to_string();

    Password {
        absolute: random_string,
        hash: password_hash,
    }
}

pub fn run_if_valid_bearer<T>(
    bearer_token: &str,
    init_if_valid: Box<dyn FnOnce() -> T>,
) -> Result<T, StatusCode> {
    let stored_hash_string = &*LIMAE_CONFIG.password_hash;
    let parsed_hash =
        PasswordHash::new(&stored_hash_string).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !Argon2::default()
        .verify_password(bearer_token.as_ref(), &parsed_hash)
        .is_ok()
    {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        Ok(init_if_valid())
    }
}

pub fn print_password(pwd: &str) {
    println!("=====");
    println!("{}", pwd.bold().bright_green());
    println!("=====");
    println!(
        "{}",
        "Make sure to store it privately, you will not be able to see your password again."
            .bright_green()
    );
}
