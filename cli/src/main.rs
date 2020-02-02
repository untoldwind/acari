use acari_lib::{AcariError, DateSpan};
use clap::{App, Arg, SubCommand};

mod commands;
mod config;

use commands::OutputFormat;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let app = App::new("acarid")
    .version("0.1")
    .about("Commandline interface for mite")
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("format")
        .help("Output format (pretty, json, flat)"),
    )
    .arg(Arg::with_name("no-cache").long("no-cache").help("Disable the use of cache files"))
    .subcommand(SubCommand::with_name("init").about("Initialize connection to mite"))
    .subcommand(SubCommand::with_name("check").about("Check connection to mite"))
    .subcommand(SubCommand::with_name("clear-cache").about("Clear the local cache"))
    .subcommand(SubCommand::with_name("customers").about("List all customers"))
    .subcommand(
      SubCommand::with_name("projects")
        .arg(Arg::with_name("customer").help("Optional: List only projects of a specific customer"))
        .about("List all projects"),
    )
    .subcommand(SubCommand::with_name("services").about("List all services"))
    .subcommand(
      SubCommand::with_name("entries")
        .arg(
          Arg::with_name("span")
            .required(true)
            .help("Date span to query\n(today, yesterday, this-week, last-week,\n this-month, last-month, yyyy-mm-dd, yyyy-mm-dd|yyyy-mm-dd)"),
        )
        .about("Query time entries"),
    );
  let matches = app.get_matches();

  let output_format = matches.value_of("output").map(OutputFormat::from_string).unwrap_or(Ok(OutputFormat::Pretty))?;

  match Config::read()? {
    Some(config) => {
      let client = config.client(!matches.is_present("no-cache"))?;
      match matches.subcommand() {
        ("init", _) => commands::init()?,
        ("check", _) => commands::check(client.as_ref(), output_format)?,
        ("clear-cache", _) => commands::clear_cache()?,
        ("customers", _) => commands::customers(client.as_ref(), output_format)?,
        ("projects", Some(sub_matches)) => match sub_matches.value_of("customer") {
          Some(customer) => commands::projects_of_customer(client.as_ref(), customer, output_format)?,
          None => commands::all_projects(client.as_ref(), output_format)?,
        },
        ("services", _) => commands::services(client.as_ref(), output_format)?,
        ("entries", Some(sub_matches)) => {
          let span_arg = sub_matches
            .value_of("span")
            .ok_or(AcariError::UserError("Missing <span> argument".to_string()))?;
          commands::entries(client.as_ref(), output_format, DateSpan::from_string(span_arg)?)?;
        }
        (invalid, _) => Err(AcariError::UserError(format!("Unknown command: {}", invalid)))?,
      }
    }
    None => match matches.subcommand() {
      ("init", _) => commands::init()?,
      (_, _) => Err(AcariError::UserError("Missing configuration, run init first".to_string()))?,
    },
  }

  Ok(())
}
