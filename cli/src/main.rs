use clap::{App, Arg, SubCommand};

mod commands;
mod config;
mod error;

use commands::OutputFormat;
use config::Config;
use error::AppError;

fn main() -> Result<(), AppError> {
  let app = App::new("acarid")
    .version("0.1")
    .about("Commandline interface for mite")
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("format")
        .help("Output format (pretty|json|flat)"),
    )
    .subcommand(SubCommand::with_name("init").about("Initialize connection to mite"))
    .subcommand(SubCommand::with_name("check").about("Check connection to mite"))
    .subcommand(SubCommand::with_name("projects").about("List all projects"))
    .subcommand(SubCommand::with_name("services").about("List all services"));
  let matches = app.get_matches();

  let output_format = matches.value_of("output").map(OutputFormat::from_string).unwrap_or(Ok(OutputFormat::Pretty))?;

  match Config::read()? {
    Some(config) => match matches.subcommand() {
      ("init", _) => commands::init(),
      ("check", _) => commands::check(&config, output_format),
      ("projects", _) => commands::all_projects(&config, output_format),
      ("services", _) => commands::services(&config, output_format),
      (invalid, _) => Err(AppError::UserError(format!("Unknown command: {}", invalid))),
    },
    None => match matches.subcommand() {
      ("init", _) => commands::init(),
      (_, _) => Err(AppError::UserError("Missing configuration, run init first".to_string())),
    },
  }
}
