use clap::{Arg, Command};

pub enum LimaeSubCommand {
    Serve,
    Rephrase,
    Downloads,
    List,
}

impl LimaeSubCommand {
    pub fn build(&self) -> Command {
        match self {
            LimaeSubCommand::Serve => {
                Command::new(LimaeSubCommand::Serve.lowercase_name())
                    .about("Serves the linguistic tools and exposes the API that the Limae App expects. Use `-p` or `--port` to configure the listening port (default is 73202).")
                    .arg(Arg::new("port").long("port").short('p').required(false).help("A valid port value").default_value("73202"))
            }
            LimaeSubCommand::Rephrase => {
                Command::new(LimaeSubCommand::Rephrase.lowercase_name())
                    .about("Directly rephrase your text within the CLI environment without needing the full Limae App.")
                    .arg(Arg::new("text").long("text").short('t').required(true).help("The text you want to rephrase"))
            }
            LimaeSubCommand::Downloads => {
                Command::new(LimaeSubCommand::Downloads.lowercase_name()).
                    about("Lists all available downloads for the current version you are using.")
            }
            LimaeSubCommand::List => {
                Command::new(LimaeSubCommand::List.lowercase_name())
                    .about("Lists the modules that have already been downloaded and are ready to use.")
            }
        }
    }

    pub fn lowercase_name(&self) -> &str {
        match self {
            LimaeSubCommand::Serve => "serve",
            LimaeSubCommand::Rephrase => "rephrase",
            LimaeSubCommand::Downloads => "downloads",
            LimaeSubCommand::List => "list",
        }
    }

    pub fn get(lowercase_name: &str) -> LimaeSubCommand {
        match lowercase_name {
            "serve" => LimaeSubCommand::Serve,
            "rephrase" => LimaeSubCommand::Rephrase,
            "list" => LimaeSubCommand::List,
            _ => LimaeSubCommand::Downloads,
        }
    }
}
