use super::OutputFormat;
use acari_lib::{AcariError, Client, DateSpan, Minutes, TimeEntry, TrackingTimeEntry};
use chrono::NaiveDate;
use itertools::Itertools;
use prettytable::{cell, format, row, Table};
use serde_json::{json, Value};

pub fn entries(client: &dyn Client, output_format: OutputFormat, date_span: DateSpan) -> Result<(), AcariError> {
  let tracker = client.get_tracker()?;
  let mut time_entries = client.get_time_entries(date_span)?;

  time_entries.sort_by(|t1, t2| t1.date_at.cmp(&t2.date_at));

  let grouped: Vec<(&NaiveDate, Vec<&TimeEntry>)> = time_entries
    .iter()
    .group_by(|e| &e.date_at)
    .into_iter()
    .map(|(customer_name, group)| (customer_name, group.collect()))
    .collect();

  match output_format {
    OutputFormat::Pretty => print_pretty(grouped, tracker.tracking_time_entry),
    OutputFormat::Json => print_json(time_entries, tracker.tracking_time_entry)?,
    OutputFormat::Flat => print_flat(grouped, tracker.tracking_time_entry),
  }

  Ok(())
}

fn print_pretty(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>, tracking_time_entry: Option<TrackingTimeEntry>) {
  let mut entries_table = Table::new();
  entries_table.set_titles(row!["Day", "Time", "Customer", "Project", "Service"]);
  entries_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

  for (day, group) in entries {
    let sum = group.iter().map(|e| e.minutes).sum::<Minutes>();
    entries_table.add_row(row![bFc -> day, bFc -> sum, H3 -> " " ]);
    for entry in group {
      if let Some(tracking_entry) = tracking_time_entry.filter(|t| t.id == entry.id) {
        entries_table.add_row(row![FY => "", tracking_entry.minutes, entry.customer_name, entry.project_name, entry.service_name]);
      } else if entry.locked {
        entries_table.add_row(row![Fr => "", entry.minutes, entry.customer_name, entry.project_name, entry.service_name]);
      } else {
        entries_table.add_row(row!["", entry.minutes, entry.customer_name, entry.project_name, entry.service_name]);
      }
    }
  }
  entries_table.printstd();
}

fn print_json(entries: Vec<TimeEntry>, tracking_time_entry: Option<TrackingTimeEntry>) -> Result<(), AcariError> {
  let json_entries: Result<Vec<Value>, AcariError> = entries
    .into_iter()
    .map(|entry| match serde_json::to_value(&entry)? {
      Value::Object(mut fields) => {
        if let Some(tracking_entry) = tracking_time_entry.filter(|t| t.id == entry.id) {
          fields.insert("tracking".to_string(), Value::Bool(true));
          fields["minutes"] = json!(tracking_entry.minutes);
        } else {
          fields.insert("tracking".to_string(), Value::Bool(false));
        }
        Ok(Value::Object(fields))
      }
      value => Ok(value),
    })
    .collect();
  println!("{}", serde_json::to_string_pretty(&json_entries?)?);

  Ok(())
}

fn print_flat(entries: Vec<(&NaiveDate, Vec<&TimeEntry>)>, tracking_time_entry: Option<TrackingTimeEntry>) {
  for (date, group) in entries {
    for entry in group {
      if let Some(tracking_entry) = tracking_time_entry.filter(|t| t.id == entry.id) {
        println!(
          "{}\t{}\t{}\t{}\t{}\tTRACKING",
          date, entry.customer_name, entry.project_name, entry.service_name, tracking_entry.minutes,
        );
      } else if entry.locked {
        println!(
          "{}\t{}\t{}\t{}\t{}\tLOCKED",
          date, entry.customer_name, entry.project_name, entry.service_name, entry.minutes,
        );
      } else {
        println!(
          "{}\t{}\t{}\t{}\t{}\tOPEN",
          date, entry.customer_name, entry.project_name, entry.service_name, entry.minutes,
        );
      }
    }
  }
}
