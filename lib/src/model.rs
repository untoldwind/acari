use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
  pub id: u32,
  pub name: String,
  pub title: String,
  pub currency: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: u32,
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
  pub id: u32,
  pub name: String,
  pub note: String,
  pub hourly_rate: u32,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
  pub id: u32,
  pub name: String,
  pub customer_id: u32,
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
  pub id: u32,
  pub name: String,
  pub note: String,
  pub hourly_rate: u32,
  pub billable: bool,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeEntry {
  pub id: u32,
  pub date_at: NaiveDate,
  pub minutes: u32,
  pub customer_id: u32,
  pub customer_name: String,
  pub project_id: u32,
  pub project_name: String,
  pub service_id: u32,
  pub service_name: String,
  pub user_id: u32,
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
