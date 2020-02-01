use super::OutputFormat;
use crate::config::Config;
use crate::error::AppError;
use acari_lib::DateSpan;
use itertools::Itertools;
use prettytable::{cell, format, row, table};

pub fn entries(config: &Config, output_format: OutputFormat, date_span: DateSpan) -> Result<(), AppError> {
  let client = config.client();
  let mut time_entries = client.get_time_entries(date_span)?;

  time_entries.sort_by(|t1, t2| t1.date_at.cmp(&t2.date_at));

  println!("{:?}", time_entries);
  Ok(())
}
