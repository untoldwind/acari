use super::OutputFormat;
use crate::config::Config;
use crate::error::AppError;
use prettytable::{cell, format, row, table};

pub fn services(config: &Config, output_format: OutputFormat) -> Result<(), AppError> {
  let client = config.client();
  let services = client.get_services()?;

  Ok(())
}
