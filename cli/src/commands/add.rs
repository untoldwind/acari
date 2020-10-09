use super::OutputFormat;
use super::{entries, find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, Day, Minutes};
use clap::Clap;

#[derive(Clap, PartialEq, Eq)]
pub struct AddCmd {
  #[clap(about = "Customer name")]
  customer: String,
  #[clap(about = "Project name")]
  project: String,
  #[clap(about = "Service name")]
  service: String,
  #[clap(about = "Time (minutes or hh:mm)")]
  time: Minutes,
  #[clap(about = "Date", default_value = "today")]
  day: Day,
  #[clap(short, long, about = "Optional note")]
  note: Option<String>,
}

impl AddCmd {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    let customer = find_customer(client, &self.customer)?;
    let project = find_project(client, customer.id, &self.project)?;
    let service = find_service(client, &self.service)?;

    client.create_time_entry(self.day, project.id, service.id, self.time, self.note.clone())?;

    entries(client, output_format, self.day.into())
  }
}
