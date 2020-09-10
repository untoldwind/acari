use acari_lib::{user_error, AcariError, Minutes};
use clap::{crate_description, crate_version, App, Arg, ArgMatches, SubCommand};
use std::str;

mod commands;
mod config;

use commands::OutputFormat;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  openssl_probe::init_ssl_cert_env_vars();

  let mut app = App::new("acari")
    .version(crate_version!())
    .about(crate_description!())
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("format")
        .help("Output format (pretty, json, flat)"),
    )
    .arg(
      Arg::with_name("profile")
        .short("p")
        .long("profile")
        .value_name("profile")
        .help("Select profile"),
    )
    .arg(Arg::with_name("no-cache").long("no-cache").help("Disable the use of cache files"))
    .subcommand(
      SubCommand::with_name("add")
        .arg(Arg::with_name("customer").required(true).help("Customer name"))
        .arg(Arg::with_name("project").required(true).help("Project name"))
        .arg(Arg::with_name("service").required(true).help("Service name"))
        .arg(Arg::with_name("time").required(true).help("Time (minutes or hh:mm)"))
        .arg(Arg::with_name("date").help("Optional: Date (default: today)"))
        .about("Just add a time entry"),
    )
    .subcommand(SubCommand::with_name("init").about("Initialize connection to mite"))
    .subcommand(SubCommand::with_name("check").about("Check connection to mite"))
    .subcommand(SubCommand::with_name("clear-cache").about("Clear the local cache"))
    .subcommand(SubCommand::with_name("customers").about("List all customers"))
    .subcommand(
      SubCommand::with_name("entries")
        .arg(
          Arg::with_name("span")
            .required(true)
            .help("Date span to query\n(today, yesterday, this-week, last-week,\n this-month, last-month, yyyy-mm-dd, yyyy-mm-dd/yyyy-mm-dd)"),
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
        .arg(Arg::with_name("date").help("Optional: Date (default: today)"))
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
    .subcommand(SubCommand::with_name("tracking").about("Show currently tracked time entry"))
    .subcommand(SubCommand::with_name("profiles").about("List configured profiles"));

  let mut help = Vec::new();
  app.write_long_help(&mut help)?;

  let matches = match app.get_matches_safe() {
    Ok(matches) => matches,
    Err(_) => {
      println!("{}", str::from_utf8(&help)?);
      return Ok(());
    }
  };

  let output_format = matches.value_of("output").map(OutputFormat::from_string).unwrap_or(Ok(OutputFormat::Pretty))?;
  let maybe_profile = matches.value_of("profile");

  if matches.subcommand_name() == Some("init") {
    // Init is special
    commands::init(Config::read()?, maybe_profile)?;
  } else if let Some(config) = Config::read()? {
    let client = config.client(maybe_profile, !matches.is_present("no-cache"))?;
    match matches.subcommand() {
      ("add", Some(sub_matches)) => {
        let customer = required_arg(sub_matches, "customer")?;
        let project = required_arg(sub_matches, "project")?;
        let service = required_arg(sub_matches, "service")?;
        let time = required_arg(sub_matches, "time")?.parse::<Minutes>()?;
        let maybe_day = sub_matches.value_of("date").map(str::parse).transpose()?;

        commands::add(client.as_ref(), output_format, customer, project, service, time, maybe_day)?;
      }
      ("check", _) => commands::check(client.as_ref(), output_format)?,
      ("clear-cache", _) => commands::clear_cache()?,
      ("customers", _) => commands::customers(client.as_ref(), output_format)?,
      ("entries", Some(sub_matches)) => {
        let span = required_arg(sub_matches, "span")?;
        commands::entries(client.as_ref(), output_format, span.parse()?)?;
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
        let time = required_arg(sub_matches, "time")?.parse::<Minutes>()?;
        let maybe_day = sub_matches.value_of("date").map(str::parse).transpose()?;

        commands::set(client.as_ref(), output_format, customer, project, service, time, maybe_day)?;
      }
      ("start", Some(sub_matches)) => {
        let customer = required_arg(sub_matches, "customer")?;
        let project = required_arg(sub_matches, "project")?;
        let service = required_arg(sub_matches, "service")?;
        let maybe_offset = sub_matches.value_of("offset").map(str::parse::<Minutes>).transpose()?;

        commands::start(client.as_ref(), output_format, customer, project, service, maybe_offset)?;
      }
      ("stop", _) => commands::stop(client.as_ref(), output_format)?,
      ("tracking", _) => commands::tracking(client.as_ref(), output_format)?,
      ("profiles", _) => commands::profiles(config),
      (_, _) => {
        println!("{}", str::from_utf8(&help)?);
      }
    }
  } else {
    return Err(AcariError::UserError("Missing configuration, run init first".to_string()).into());
  }

  Ok(())
}

fn required_arg<'a>(matches: &'a ArgMatches, name: &str) -> Result<&'a str, AcariError> {
  matches.value_of(name).ok_or_else(|| user_error!("Missing <{}> argument", name))
}
