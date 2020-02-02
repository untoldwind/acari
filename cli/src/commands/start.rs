use super::OutputFormat;
use super::{find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, DateSpan, Minutes};
use prettytable::{cell, format, row, Table};

pub fn start(
  client: &dyn Client,
  output_format: OutputFormat,
  customer_name: &str,
  project_name: &str,
  service_name: &str,
  minutes_offset: Option<Minutes>,
) -> Result<(), AcariError> {
  let customer = find_customer(client, customer_name)?;
  let project = find_project(client, customer.id, project_name)?;
  let service = find_service(client, service_name)?;

  let existing = match minutes_offset {
    Some(_) => None,
    None => {
      let mut time_entries = client.get_time_entries(DateSpan::Today)?;

      time_entries
        .into_iter()
        .find(|e| e.customer_id == customer.id && e.project_id == project.id && e.service_id == service.id)
    }
  };

  Ok(())
}
