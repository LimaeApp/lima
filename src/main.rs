#[path = "cli/cli.rs"]
mod cli;
use crate::cli::start_cli;
pub mod constants;

fn main() {
    start_cli()
}
