use super::OutputFormat;
use acari_lib::{AcariError, Client, Tracker};
use prettytable::{cell, format, row, Table};

pub fn stop(client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
  Ok(())
}
