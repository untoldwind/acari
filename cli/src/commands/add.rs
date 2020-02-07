use super::OutputFormat;
use super::{entries, find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, Day, Minutes};

pub fn add(
  client: &dyn Client,
  output_format: OutputFormat,
  customer_name: &str,
  project_name: &str,
  service_name: &str,
  minutes: Minutes,
  maybe_day: Option<Day>,
) -> Result<(), AcariError> {
  let customer = find_customer(client, customer_name)?;
  let project = find_project(client, customer.id, project_name)?;
  let service = find_service(client, service_name)?;
  let date = maybe_day.unwrap_or(Day::Today).as_date();

  client.create_time_entry(maybe_day.unwrap_or(Day::Today), project.id, service.id, minutes)?;

  entries(client, output_format, date.into())
}