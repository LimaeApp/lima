#[path = "cli/serve/apps_router.rs"]
mod apps_router;
#[path = "cli/cli.rs"]
mod cli;
mod configuration;
#[path = "cli/serve/dictionary_router.rs"]
mod dictionary_router;
pub mod models;
#[path = "cli/serve/notes_router.rs"]
mod notes_router;
#[path = "cli/serve/serve.rs"]
mod serve;
pub mod utils;

use crate::cli::start_cli;
use crate::configuration::LimaeConfiguration;
use std::env;
use std::sync::LazyLock;

pub const CLI_VERSION: &str = "0.1.0";
pub const PROJ_NAME: &str = "Lima";

pub const PAGE_ID: &str = "page_id";
pub const PAGE_SIZE: &str = "page_size";
pub const DEFAULT_PAGE_SIZE: &u32 = &15;

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
