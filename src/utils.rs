use crate::LIMAE_CONFIG;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::Json;
use axum::http::StatusCode;
use colored::Colorize;
use serde::Serialize;
use serde_json::Value;
use sqlx::Error;
use sqlx::sqlite::SqliteQueryResult;

pub async fn run_if_valid_bearer<Type, Function, Deferred>(
    bearer_token: &str,
    init_if_valid: Function,
) -> Result<Type, StatusCode>
where
    Function: FnOnce() -> Deferred,
    Deferred: Future<Output = Result<Type, StatusCode>>,
{
    let stored_hash_string = &*LIMAE_CONFIG.password_hash;
    let parsed_hash =
        PasswordHash::new(&stored_hash_string).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !Argon2::default()
        .verify_password(bearer_token.as_ref(), &parsed_hash)
        .is_ok()
    {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        Ok(init_if_valid().await?)
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

pub trait SerializeResultToJson {
    fn to_json_result(self) -> Result<Json<Value>, StatusCode>;
}

pub trait ResultToJson {
    fn to_json_result(self, response: String) -> Result<Json<Value>, StatusCode>;
}

impl ResultToJson for Result<SqliteQueryResult, Error> {
    fn to_json_result(self, response: String) -> Result<Json<Value>, StatusCode> {
        match self {
            Ok(_) => Ok(Json(Value::String(response))),
            Err(error) => {
                println!("{}", error);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}

impl<T: Serialize> SerializeResultToJson for Result<Vec<T>, Error> {
    fn to_json_result(self) -> Result<Json<Value>, StatusCode> {
        match self {
            Ok(vec_result) => Ok(Json(
                serde_json::to_value(&vec_result).unwrap_or(Value::Null),
            )),
            Err(error) => {
                println!("{}", error);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
