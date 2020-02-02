use acari_lib::{user_error, AcariError, DateSpan};
use clap::{App, Arg, ArgMatches, SubCommand};

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
      SubCommand::with_name("entries")
        .arg(
          Arg::with_name("span")
            .required(true)
            .help("Date span to query\n(today, yesterday, this-week, last-week,\n this-month, last-month, yyyy-mm-dd, yyyy-mm-dd|yyyy-mm-dd)"),
        )
        .about("Query time entries"),
    )
    .subcommand(
      SubCommand::with_name("projects")
        .arg(Arg::with_name("customer").help("Optional: List only projects of a specific customer"))
        .about("List all projects"),
    )
    .subcommand(SubCommand::with_name("services").about("List all services"))
    .subcommand(
      SubCommand::with_name("set")
        .arg(Arg::with_name("customer").required(true).help("Customer name"))
        .arg(Arg::with_name("project").required(true).help("Project name"))
        .arg(Arg::with_name("service").required(true).help("Service name"))
        .arg(Arg::with_name("time").required(true).help("Time (minutes or hh:mm)"))
        .about("Start tracking time"),
    )
    .subcommand(
      SubCommand::with_name("start")
        .arg(Arg::with_name("customer").required(true).help("Customer name"))
        .arg(Arg::with_name("project").required(true).help("Project name"))
        .arg(Arg::with_name("service").required(true).help("Service name"))
        .arg(Arg::with_name("offset").help("Optional: Starting offset (minutes or hh:mm)"))
        .about("Start tracking time"),
    )
    .subcommand(SubCommand::with_name("stop").about("Stop current time tracking"))
    .subcommand(SubCommand::with_name("tracking").about("Show currently tracked time entry"));

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
        ("entries", Some(sub_matches)) => {
          let span = required_arg(sub_matches, "span")?;
          commands::entries(client.as_ref(), output_format, DateSpan::from_string(span)?)?;
        }
        ("projects", Some(sub_matches)) => match sub_matches.value_of("customer") {
          Some(customer) => commands::projects_of_customer(client.as_ref(), output_format, customer)?,
          None => commands::all_projects(client.as_ref(), output_format)?,
        },
        ("services", _) => commands::services(client.as_ref(), output_format)?,
        ("set", Some(sub_matches)) => {
          let customer = required_arg(sub_matches, "customer")?;
          let project = required_arg(sub_matches, "project")?;
          let service = required_arg(sub_matches, "service")?;
          let time = parse_minutes(required_arg(sub_matches, "time")?)?;

          commands::set(client.as_ref(), output_format, customer, project, service, time)?;
        }
        ("start", Some(sub_matches)) => {
          let customer = required_arg(sub_matches, "customer")?;
          let project = required_arg(sub_matches, "project")?;
          let service = required_arg(sub_matches, "service")?;
          let offset = minutes_arg(sub_matches, "offset")?;

          commands::start(client.as_ref(), output_format, customer, project, service, offset.unwrap_or(0))?;
        }
        ("stop", _) => commands::stop(client.as_ref(), output_format)?,
        ("tracking", _) => commands::tracking(client.as_ref(), output_format)?,
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

fn required_arg<'a>(matches: &'a ArgMatches, name: &str) -> Result<&'a str, AcariError> {
  matches.value_of(name).ok_or_else(|| user_error!("Missing <{}> argument", name))
}

fn minutes_arg(matches: &ArgMatches, name: &str) -> Result<Option<u32>, AcariError> {
  match matches.value_of(name) {
    Some(value) => parse_minutes(value).map(Some),
    None => Ok(None),
  }
}

fn parse_minutes(expr: &str) -> Result<u32, AcariError> {
  match expr.find(":") {
    Some(idx) => {
      let hours = expr[..idx].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;
      let minutes = expr[idx + 1..].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;

      Ok(hours * 60 + minutes)
    }
    None => expr.parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e)),
  }
}
