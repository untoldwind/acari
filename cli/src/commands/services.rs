use super::OutputFormat;
use crate::config::Config;
use crate::error::AppError;
use acari_lib::Service;
use prettytable::{cell, format, row, table};

pub fn services(config: &Config, output_format: OutputFormat) -> Result<(), AppError> {
  let client = config.client();
  let mut services = client.get_services()?;

  services.sort_by(|s1, s2| s1.name.cmp(&s2.name));

  match output_format {
    OutputFormat::Pretty => print_pretty(services),
    OutputFormat::Json => print_json(services)?,
    OutputFormat::Flat => print_flat(services),
  }

  Ok(())
}

fn print_pretty(services: Vec<Service>) {}

fn print_json(services: Vec<Service>) -> Result<(), AppError> {
  println!("{}", serde_json::to_string_pretty(&services)?);

  Ok(())
}

fn print_flat(services: Vec<Service>) {
  for service in services {
    println!("{}", service.name);
  }
}
