use super::OutputFormat;
use super::{find_customer, find_project, find_service};
use acari_lib::{AcariError, Client, Day, Minutes, TimeEntry, Tracker};
use clap::Args;
use prettytable::{format, row, table};
use serde_json::json;

#[derive(Debug, Args, PartialEq, Eq)]
pub struct StartCmd {
  #[clap(help = "Customer name")]
  customer: String,
  #[clap(help = "Project name")]
  project: String,
  #[clap(help = "Service name")]
  service: String,
  #[clap(help = "Optional: Starting offset (minutes or hh:mm)")]
  offset: Option<Minutes>,
  #[clap(short, long, help = "Optional note")]
  note: Option<String>,
}

impl StartCmd {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    let customer = find_customer(client, &self.customer)?;
    let project = find_project(client, &customer.id, &self.project)?;
    let service = find_service(client, &project.id, &self.service)?;
    let date = Day::Today.as_date();

    let maybe_existing = match self.offset {
      Some(_) => None,
      None => {
        let mut existing: Vec<TimeEntry> = client
          .get_time_entries(date.into())?
          .into_iter()
          .filter(|e| e.date_at == date && e.customer_id == customer.id && e.project_id == project.id && e.service_id == service.id)
          .collect();

        existing.sort_by(|e1, e2| e2.created_at.cmp(&e1.created_at));

        existing.into_iter().next()
      }
    };
    let entry = match maybe_existing {
      Some(existing) => existing,
      None => client.create_time_entry(date.into(), &project.id, &service.id, self.offset.unwrap_or_default(), self.note.clone())?,
    };
    let tracker = client.create_tracker(&entry.id)?;

    match output_format {
      OutputFormat::Pretty => print_pretty(Some(&entry), &tracker),
      OutputFormat::Json => print_json(Some(&entry), &tracker)?,
      OutputFormat::Flat => print_flat(Some(&entry), &tracker),
    }

    Ok(())
  }
}

pub fn tracking(client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
  let tracker = client.get_tracker()?;
  let maybe_entry = if let Some(tracking_entry) = &tracker.tracking_time_entry {
    Some(tracking_entry)
  } else if let Some(tracking_entry) = &tracker.stopped_time_entry {
    Some(tracking_entry)
  } else {
    None
  };

  match output_format {
    OutputFormat::Pretty => print_pretty(maybe_entry, &tracker),
    OutputFormat::Json => print_json(maybe_entry, &tracker)?,
    OutputFormat::Flat => print_flat(maybe_entry, &tracker),
  }

  Ok(())
}

pub fn stop(client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
  let current_tracker = client.get_tracker()?;
  let (update_tracker, maybe_entry) = if let Some(tracking_entry) = &current_tracker.tracking_time_entry {
    (client.delete_tracker(&tracking_entry.id)?, Some(tracking_entry))
  } else if let Some(tracking_entry) = &current_tracker.stopped_time_entry {
    (current_tracker.clone(), Some(tracking_entry))
  } else {
    (current_tracker, None)
  };

  match output_format {
    OutputFormat::Pretty => print_pretty(maybe_entry, &update_tracker),
    OutputFormat::Json => print_json(maybe_entry, &update_tracker)?,
    OutputFormat::Flat => print_flat(maybe_entry, &update_tracker),
  }

  Ok(())
}

fn print_pretty(maybe_entry: Option<&TimeEntry>, tracker: &Tracker) {
  match maybe_entry {
    Some(entry) => {
      let mut entry_table = table!(
        ["Day", entry.date_at],
        ["Customer", entry.customer_name],
        ["Project", entry.project_name],
        ["Service", entry.service_name]
      );
      entry_table.set_format(*format::consts::FORMAT_CLEAN);

      if let Some(tracking_entry) = tracker.tracking_time_entry.as_ref().filter(|t| t.id == entry.id) {
        entry_table.add_row(row![FY => "Time", tracking_entry.minutes]);

        match tracker.since {
          Some(since) => println!("Currently tracking since {}", since),
          None => println!("Currently tracking"),
        }
        entry_table.printstd();
      } else if let Some(tracking_entry) = tracker.stopped_time_entry.as_ref().filter(|t| t.id == entry.id) {
        entry_table.add_row(row!["Time", tracking_entry.minutes]);
        println!("Stooped tracking");
        entry_table.printstd();
      } else {
        println!("Currently not tracking anything");
      }
    }
    None => println!("Currently not tracking anything"),
  }
}

fn print_json(maybe_entry: Option<&TimeEntry>, tracker: &Tracker) -> Result<(), AcariError> {
  match maybe_entry {
    Some(entry) => {
      if let Some(tracking_entry) = tracker.tracking_time_entry.as_ref().filter(|t| t.id == entry.id) {
        println!("{}", serde_json::to_string_pretty(&json!({ "entry": entry, "tracking": tracking_entry }))?);
      } else if tracker.stopped_time_entry.as_ref().filter(|t| t.id == entry.id).is_some() {
        println!("{}", serde_json::to_string_pretty(&json!({ "stopped": entry }))?);
      } else {
        println!("{}", serde_json::to_string_pretty(&json!({}))?);
      }
    }
    None => println!("{}", serde_json::to_string_pretty(&json!({}))?),
  }
  Ok(())
}

fn print_flat(maybe_entry: Option<&TimeEntry>, tracker: &Tracker) {
  match maybe_entry {
    Some(entry) => {
      if let Some(tracking_entry) = tracker.tracking_time_entry.as_ref().filter(|t| t.id == entry.id) {
        println!(
          "Tracking {}\t{}\t{}\t{}\t{}",
          entry.date_at, entry.customer_name, entry.project_name, entry.service_name, tracking_entry.minutes,
        );
      } else if tracker.stopped_time_entry.as_ref().filter(|t| t.id == entry.id).is_some() {
        println!(
          "Stopped {}\t{}\t{}\t{}\t{}",
          entry.date_at, entry.customer_name, entry.project_name, entry.service_name, entry.minutes,
        );
      } else {
        println!("NotTracking");
      }
    }
    None => println!("NotTracking"),
  }
}
