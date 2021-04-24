use super::OutputFormat;
use super::{find_customer, find_project};
use acari_lib::{AcariError, Client, Service};
use clap::Clap;
use itertools::Itertools;
use prettytable::{cell, row, table};

#[derive(Clap, PartialEq, Eq)]
pub struct ServicesCommand {
  #[clap(about = "Customer name")]
  customer: String,
  #[clap(about = "Project name")]
  project: String,
}

impl ServicesCommand {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    let customer = find_customer(client, &self.customer)?;
    let project = find_project(client, &customer.id, &self.project)?;
    let mut services = client.get_services(&project.id)?;

    services.sort_by(|s1, s2| s1.name.cmp(&s2.name));

    match output_format {
      OutputFormat::Pretty => print_pretty(services),
      OutputFormat::Json => print_json(services)?,
      OutputFormat::Flat => print_flat(services),
    }

    Ok(())
  }
}

fn print_pretty(services: Vec<Service>) {
  let service_table = table!(
    ["Billable services"],
    [services.iter().filter(|s| s.billable && !s.archived).map(|s| &s.name).join("\n")],
    ["Not billable services"],
    [services.iter().filter(|s| !s.billable && !s.archived).map(|s| &s.name).join("\n")],
    ["Archived"],
    [services.iter().filter(|s| s.archived).map(|s| &s.name).join("\n")]
  );
  service_table.printstd();
}

fn print_json(services: Vec<Service>) -> Result<(), AcariError> {
  println!("{}", serde_json::to_string_pretty(&services)?);

  Ok(())
}

fn print_flat(services: Vec<Service>) {
  for service in services {
    if !service.archived {
      println!("{}", service.name);
    }
  }
}
