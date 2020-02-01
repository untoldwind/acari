use clap::{App, Arg, SubCommand};

mod commands;
mod config;
mod error;

use config::Config;
use error::AppError;

fn main() -> Result<(), AppError> {
  let app = App::new("acarid")
    .version("0.1")
    .about("Commandline interface for mite")
    .subcommand(SubCommand::with_name("init").about("Initialize connection to mite"))
    .subcommand(SubCommand::with_name("check").about("Check connection to mite"));
  let matches = app.get_matches();

  match Config::read()? {
    Some(config) => match matches.subcommand() {
      ("init", _) => commands::init(),
      ("check", _) => commands::check(&config),
      (invalid, _) => Err(AppError::UserError(format!("Unknown command: {}", invalid))),
    },
    None => match matches.subcommand() {
      ("init", _) => commands::init(),
      (_, _) => Err(AppError::UserError("Missing configuration, run init first".to_string())),
    },
  }
}
