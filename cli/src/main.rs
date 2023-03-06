use acari_lib::{clear_cache, AcariError};
use clap::{Parser, Subcommand};
use std::str;

mod commands;
mod config;

use commands::OutputFormat;
use config::Config;

#[derive(Debug, Parser)]
#[clap(version = "0.1.10")]
struct Opts {
  #[clap(short, long, help = "Output format", default_value = "pretty")]
  output: OutputFormat,

  #[clap(short, long, help = "Select profile")]
  profile: Option<String>,

  #[clap(long, help = "Disable the use of cache files")]
  no_cache: bool,

  #[clap(subcommand)]
  subcommand: AcariSubCommand,
}

#[derive(Debug, Subcommand, PartialEq, Eq)]
enum AcariSubCommand {
  #[clap(about = "Initialize connection to mite")]
  Init,
  #[clap(about = "Just add a time entry")]
  Add(commands::AddCmd),
  #[clap(about = "Check connection to mite")]
  Check,
  #[clap(about = "Clear the local cache")]
  ClearCache,
  #[clap(about = "List all customers")]
  Customers,
  #[clap(about = "Query time entries")]
  Entries(commands::EntriesCmd),
  #[clap(about = "List configured profiles")]
  Profiles,
  #[clap(about = "List all projects")]
  Projects(commands::ProjectsCmd),
  #[clap(about = "List all services")]
  Services(commands::ServicesCommand),
  #[clap(about = "Set time for a project at specific day")]
  Set(commands::SetCmd),
  #[clap(about = "Start tracking time")]
  Start(commands::StartCmd),
  #[clap(about = "Stop current time tracking")]
  Stop,
  #[clap(about = "Show currently tracked time entry")]
  Tracking,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  openssl_probe::init_ssl_cert_env_vars();

  let opts: Opts = Opts::parse();

  if opts.subcommand == AcariSubCommand::Init {
    // Init is special
    commands::init(Config::read()?, &opts.profile)?;
  } else if let Some(config) = Config::read()? {
    let client = config.client(&opts.profile, !opts.no_cache)?;
    match opts.subcommand {
      AcariSubCommand::Add(add_cmd) => add_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Check => commands::check(client.as_ref(), opts.output)?,
      AcariSubCommand::ClearCache => clear_cache()?,
      AcariSubCommand::Customers => commands::customers(client.as_ref(), opts.output)?,
      AcariSubCommand::Entries(entries_cmd) => entries_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Profiles => commands::profiles(config),
      AcariSubCommand::Projects(projects_cmd) => projects_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Services(services_cmd) => services_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Set(set_cmd) => set_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Start(start_cmd) => start_cmd.run(client.as_ref(), opts.output)?,
      AcariSubCommand::Stop => commands::stop(client.as_ref(), opts.output)?,
      AcariSubCommand::Tracking => commands::tracking(client.as_ref(), opts.output)?,
      AcariSubCommand::Init => unreachable!(),
    }
  } else {
    return Err(AcariError::UserError("Missing configuration, run init first".to_string()).into());
  }

  Ok(())
}
