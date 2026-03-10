#[path = "cli/cli.rs"]
mod cli;
#[path = "cli/serve/route.rs"]
mod route;
#[path = "cli/serve/serve.rs"]
mod serve;
use crate::cli::start_cli;
pub mod constants;

fn main() {
    start_cli()
}
