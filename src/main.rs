#[path = "cli/cli.rs"]
mod cli;
#[path = "cli/serve/route.rs"]
mod route;
#[path = "cli/serve/serve.rs"]
mod serve;

use crate::cli::start_cli;
use std::env;
use std::sync::LazyLock;

pub const CLI_VERSION: &str = "0.1.0";
pub const PROJ_NAME: &str = "Lima";
pub static LIMAE_DIR_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/.limae", env::home_dir().unwrap().display()));

fn main() {
    start_cli()
}
