use super::OutputFormat;
use crate::error::AppError;
use acari_lib::{Client, DateSpan, TimeEntry};
use chrono::NaiveDate;
use itertools::Itertools;
use prettytable::{cell, row, table};

pub fn entries(client: &dyn Client, output_format: OutputFormat, date_span: DateSpan) -> Result<(), AppError> {
  let mut time_entries = client.get_time_entries(date_span)?;

  time_entries.sort_by(|t1, t2| t1.date_at.cmp(&t2.date_at));

  let grouped: Vec<(&NaiveDate, Vec<&TimeEntry>)> = time_entries
    .iter()
    .group_by(|e| &e.date_at)
    .into_iter()
    .map(|(customer_name, group)| (customer_name, group.collect()))
    .collect();

  match output_format {
    OutputFormat::Pretty => print_pretty(grouped),
    OutputFormat::Json => print_json(time_entries)?,
    OutputFormat::Flat => print_flat(grouped),
  }

  Ok(())
}

fn print_pretty(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>) {
  let mut entries_table = table!(["Customer", "Projects"]);

  for (customer_name, group) in entries {}
  entries_table.printstd();
}

fn print_json(entries: Vec<TimeEntry>) -> Result<(), AppError> {
  println!("{}", serde_json::to_string_pretty(&entries)?);

  Ok(())
}

fn print_flat(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>) {
  for (date, group) in entries {
    for entry in group {
      println!(
        "{}\t{}\t{}\t{}\t{}",
        date,
        entry.customer_name,
        entry.project_name,
        entry.service_name,
        render_minutes(entry.minutes)
      );
    }
  }
}

fn render_minutes(minutes: u32) -> String {
  format!("{}:{:02}", minutes / 60, minutes % 60)
}
