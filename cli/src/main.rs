use acari_lib::{clear_cache, AcariError};
use clap::Clap;
use std::str;

mod commands;
mod config;

use commands::OutputFormat;
use config::Config;

#[derive(Clap)]
#[clap(version = "0.1.9")]
struct Opts {
  #[clap(short, long, arg_enum, about = "Output format", default_value = "pretty")]
  output: OutputFormat,

  #[clap(short, long, about = "Select profile")]
  profile: Option<String>,

  #[clap(long, about = "Disable the use of cache files")]
  no_cache: bool,

  #[clap(subcommand)]
  subcommand: SubCommand,
}

#[derive(Clap, PartialEq, Eq)]
enum SubCommand {
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

  if opts.subcommand == SubCommand::Init {
    // Init is special
    commands::init(Config::read()?, &opts.profile)?;
  } else if let Some(config) = Config::read()? {
    let client = config.client(&opts.profile, !opts.no_cache)?;
    match opts.subcommand {
      SubCommand::Add(add_cmd) => add_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Check => commands::check(client.as_ref(), opts.output)?,
      SubCommand::ClearCache => clear_cache()?,
      SubCommand::Customers => commands::customers(client.as_ref(), opts.output)?,
      SubCommand::Entries(entries_cmd) => entries_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Profiles => commands::profiles(config),
      SubCommand::Projects(projects_cmd) => projects_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Services(services_cmd) => services_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Set(set_cmd) => set_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Start(start_cmd) => start_cmd.run(client.as_ref(), opts.output)?,
      SubCommand::Stop => commands::stop(client.as_ref(), opts.output)?,
      SubCommand::Tracking => commands::tracking(client.as_ref(), opts.output)?,
      SubCommand::Init => unreachable!(),
    }
  } else {
    return Err(AcariError::UserError("Missing configuration, run init first".to_string()).into());
  }

  Ok(())
}
