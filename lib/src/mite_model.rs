use crate::{
  model::{Account, AccountId, Customer, CustomerId, Minutes, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, User, UserId},
  DateSpan, Day,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteAccount {
  pub id: AccountId,
  pub name: String,
  pub title: String,
  pub currency: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<MiteAccount> for Account {
  fn from(f: MiteAccount) -> Self {
    Account {
      id: f.id,
      name: f.name,
      title: f.title,
      currency: f.currency,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteUser {
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

impl From<MiteUser> for User {
  fn from(f: MiteUser) -> Self {
    User {
      id: f.id,
      name: f.name,
      email: f.email,
      note: f.note,
      role: f.role,
      language: f.language,
      archived: f.archived,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteCustomer {
  pub id: CustomerId,
  pub name: String,
  pub note: String,
  pub hourly_rate: Option<u32>,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<MiteCustomer> for Customer {
  fn from(f: MiteCustomer) -> Self {
    Customer {
      id: f.id,
      name: f.name,
      note: f.note,
      archived: f.archived,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteProject {
  pub id: ProjectId,
  pub name: String,
  pub customer_id: CustomerId,
  pub customer_name: String,
  pub note: String,
  pub budget: u32,
  pub budget_type: String,
  pub hourly_rate: Option<u32>,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<MiteProject> for Project {
  fn from(f: MiteProject) -> Self {
    Project {
      id: f.id,
      name: f.name,
      customer_id: f.customer_id,
      customer_name: f.customer_name,
      note: f.note,
      archived: f.archived,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteService {
  pub id: ServiceId,
  pub name: String,
  pub note: String,
  pub hourly_rate: Option<u32>,
  pub billable: bool,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<MiteService> for Service {
  fn from(f: MiteService) -> Self {
    Service {
      id: f.id,
      name: f.name,
      note: f.note,
      billable: f.billable,
      archived: f.archived,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MiteTimeEntry {
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

impl From<MiteTimeEntry> for TimeEntry {
  fn from(f: MiteTimeEntry) -> Self {
    TimeEntry {
      id: f.id,
      date_at: f.date_at,
      minutes: f.minutes,
      customer_id: f.customer_id,
      customer_name: f.customer_name,
      project_id: f.project_id,
      project_name: f.project_name,
      service_id: f.service_id,
      service_name: f.service_name,
      user_id: f.user_id,
      user_name: f.user_name,
      note: f.note,
      billable: f.billable,
      locked: f.locked,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MiteTrackingTimeEntry {
  pub id: TimeEntryId,
  pub minutes: Minutes,
  pub since: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MiteTracker {
  pub tracking_time_entry: Option<MiteTrackingTimeEntry>,
  pub stopped_time_entry: Option<MiteTrackingTimeEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MiteEntity {
  Account(MiteAccount),
  User(MiteUser),
  Customer(MiteCustomer),
  Project(MiteProject),
  Service(MiteService),
  TimeEntry(MiteTimeEntry),
  Tracker(MiteTracker),
  Error(String),
}

pub fn day_query_param(day: &Day) -> String {
  match day {
    Day::Today => "today".to_string(),
    Day::Yesterday => "yesterday".to_string(),
    Day::Date(date) => format!("{}", date),
  }
}

pub fn date_span_query_param(span: &DateSpan) -> String {
  match span {
    DateSpan::ThisWeek => "at=this_week".to_string(),
    DateSpan::LastWeek => "at=last_week".to_string(),
    DateSpan::ThisMonth => "at=this_month".to_string(),
    DateSpan::LastMonth => "at=last_month".to_string(),
    DateSpan::Day(date) => format!("at={}", day_query_param(&date)),
    DateSpan::FromTo(from, to) => format!("from={}&to={}", from, to),
  }
}
