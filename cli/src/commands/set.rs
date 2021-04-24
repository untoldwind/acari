use super::OutputFormat;
use super::{entries, find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, Day, Minutes};
use clap::Clap;

#[derive(Clap, PartialEq, Eq)]
pub struct SetCmd {
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

impl SetCmd {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    let customer = find_customer(client, &self.customer)?;
    let project = find_project(client, &customer.id, &self.project)?;
    let service = find_service(client, &project.id, &self.service)?;
    let date = self.day.as_date();
    let mut time_entries = client.get_time_entries(date.into())?;

    time_entries.retain(|e| e.date_at == date && e.customer_id.eq(&customer.id) && e.project_id.eq(&project.id) && e.service_id.eq(&service.id));

    if let Some(first) = time_entries.first() {
      client.update_time_entry(&first.id, self.time, self.note.clone())?;
      for remaining in &time_entries[1..] {
        client.delete_time_entry(&remaining.id)?;
      }
    } else {
      client.create_time_entry(self.day, &project.id, &service.id, self.time, self.note.clone())?;
    }

    entries(client, output_format, date.into())
  }
}
