use super::OutputFormat;
use super::{entries, find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, Day, Minutes};
use clap::Args;

#[derive(Debug, Args, PartialEq, Eq)]
pub struct AddCmd {
  #[clap(help = "Customer name")]
  customer: String,
  #[clap(help = "Project name")]
  project: String,
  #[clap(help = "Service name")]
  service: String,
  #[clap(help = "Time (minutes or hh:mm)")]
  time: Minutes,
  #[clap(help = "Date", default_value = "today")]
  day: Day,
  #[clap(short, long, help = "Optional note")]
  note: Option<String>,
}

impl AddCmd {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    let customer = find_customer(client, &self.customer)?;
    let project = find_project(client, &customer.id, &self.project)?;
    let service = find_service(client, &project.id, &self.service)?;

    client.create_time_entry(self.day, &project.id, &service.id, self.time, self.note.clone())?;

    entries(client, output_format, self.day.into())
  }
}
