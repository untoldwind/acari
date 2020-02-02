use crate::error::AcariError;
use crate::user_error;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! id_wrapper {
  ($name: ident) => {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
    #[serde(transparent)]
    pub struct $name(u32);

    impl fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
      }
    }
  };
}

id_wrapper!(AccountId);
id_wrapper!(UserId);
id_wrapper!(CustomerId);
id_wrapper!(ProjectId);
id_wrapper!(ServiceId);
id_wrapper!(TimeEntryId);

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
  pub id: AccountId,
  pub name: String,
  pub title: String,
  pub currency: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: UserId,
  pub name: String,
  pub email: String,
  pub note: String,
  pub role: String,
  pub language: String,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
  pub id: CustomerId,
  pub name: String,
  pub note: String,
  pub hourly_rate: u32,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
  pub id: ProjectId,
  pub name: String,
  pub customer_id: CustomerId,
  pub customer_name: String,
  pub note: String,
  pub budget: u32,
  pub budget_type: String,
  pub hourly_rate: u32,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
  pub id: ServiceId,
  pub name: String,
  pub note: String,
  pub hourly_rate: Option<u32>,
  pub billable: bool,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Minutes(u32);

impl Minutes {
  pub fn from_string(expr: &str) -> Result<Minutes, AcariError> {
    match expr.find(":") {
      Some(idx) => {
        let hours = expr[..idx].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;
        let minutes = expr[idx + 1..].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;

        Ok(Minutes(hours * 60 + minutes))
      }
      None => Ok(Minutes(expr.parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?)),
    }
  }
}

impl fmt::Display for Minutes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{:02}", self.0 / 60, self.0 % 60)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntry {
  pub id: TimeEntryId,
  pub date_at: NaiveDate,
  pub minutes: Minutes,
  pub customer_id: CustomerId,
  pub customer_name: String,
  pub project_id: ProjectId,
  pub project_name: String,
  pub service_id: ServiceId,
  pub service_name: String,
  pub user_id: UserId,
  pub user_name: String,
  pub note: String,
  pub billable: bool,
  pub locked: bool,
  pub hourly_rate: u32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingTimeEntry {
  pub id: u32,
  pub minutes: u32,
  pub since: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracker {
  pub tracking_time_entry: Option<TrackingTimeEntry>,
  pub stopped_time_entry: Option<TrackingTimeEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MiteEntity {
  Account(Account),
  User(User),
  Customer(Customer),
  Project(Project),
  Service(Service),
  TimeEntry(TimeEntry),
  Tracker(Tracker),
  Error(String),
}
