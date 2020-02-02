use super::OutputFormat;
use acari_lib::{AcariError, Client, Tracker};
use prettytable::{cell, format, row, Table};

pub fn set(
  client: &dyn Client,
  output_format: OutputFormat,
  customer_name: &str,
  project_name: &str,
  service_name: &str,
  minutes: u32,
) -> Result<(), AcariError> {
  Ok(())
}
