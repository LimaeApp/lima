#[path = "cli/cli.rs"]
mod cli;
mod configuration;
pub mod helper_functions;
#[path = "cli/serve/route.rs"]
mod route;
#[path = "cli/serve/serve.rs"]
mod serve;

use crate::cli::start_cli;
use crate::configuration::LimaeConfiguration;
use std::env;
use std::sync::LazyLock;

pub const CLI_VERSION: &str = "0.1.0";
pub const PROJ_NAME: &str = "Lima";

pub static LIMAE_DIR_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/.limae", env::home_dir().unwrap().display()));

pub static LIMAE_CONFIG_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", &*LIMAE_DIR_PATH, "/cli_config.json"));

pub static LIMAE_CONFIG: LazyLock<LimaeConfiguration> = LazyLock::new(|| {
    let config_string = std::fs::read_to_string(&*LIMAE_CONFIG_PATH).unwrap();

    serde_json::from_str::<LimaeConfiguration>(&config_string)
        .expect("Couldn't deserialize the config file")
});

#[tokio::main]
async fn main() {
    start_cli().await;
}
