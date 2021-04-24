use std::collections::HashMap;

use crate::{
  model::{Account, AccountId, Customer, CustomerId, Minutes, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, User, UserId},
  AcariError, DateSpan, Day,
};
use chrono::offset::Local;
use chrono::{DateTime, Datelike, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EverhourError {
  pub code: u16,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EverhourCurrency {
  pub code: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourTeam {
  pub id: AccountId,
  pub name: String,
  pub currency_details: EverhourCurrency,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourUser {
  pub id: UserId,
  pub name: String,
  pub email: String,
  pub status: String,
  pub role: String,
  pub headline: String,
  pub is_suspended: bool,
  pub team: EverhourTeam,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
}

impl From<EverhourUser> for Account {
  fn from(f: EverhourUser) -> Self {
    Account {
      id: f.team.id,
      name: f.team.name.clone(),
      title: f.team.name,
      currency: f.team.currency_details.code,
      created_at: f.team.created_at,
    }
  }
}

impl From<EverhourUser> for User {
  fn from(f: EverhourUser) -> Self {
    User {
      id: f.id,
      name: f.name,
      email: f.email,
      role: f.role,
      note: f.headline,
      language: "".to_string(),
      archived: f.is_suspended,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourProject {
  pub id: ProjectId,
  pub name: String,
  pub workspace_id: CustomerId,
  pub workspace_name: String,
  pub status: String,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
}

impl From<EverhourProject> for Customer {
  fn from(f: EverhourProject) -> Self {
    Customer {
      id: f.workspace_id,
      name: f.workspace_name,
      note: "".to_string(),
      archived: f.status != "open",
      created_at: f.created_at,
    }
  }
}

impl From<EverhourProject> for Project {
  fn from(f: EverhourProject) -> Self {
    Project {
      id: f.id,
      name: f.name,
      note: "".to_string(),
      customer_id: f.workspace_id,
      customer_name: f.workspace_name,
      archived: f.status != "open",
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourTask {
  pub id: ServiceId,
  pub name: String,
  pub status: String,
  pub iteration: String,
  pub projects: Vec<ProjectId>,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
}

impl From<EverhourTask> for Service {
  fn from(f: EverhourTask) -> Self {
    Service {
      id: f.id,
      name: f.name,
      note: f.iteration,
      archived: f.status != "open",
      billable: true,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourTimeEntry {
  pub date: NaiveDate,
  #[serde(default)]
  pub comment: String,
  pub task: Option<EverhourTask>,
  #[serde(with = "minutes_in_seconds")]
  pub time: Minutes,
  pub user: UserId,
  pub is_locked: bool,
  #[serde(with = "date_format")]
  pub created_at: DateTime<Utc>,
}

impl EverhourTimeEntry {
  pub fn into_entry(self, project_map: &HashMap<ProjectId, EverhourProject>, user: &EverhourUser) -> Option<TimeEntry> {
    match self.task {
      Some(task) => match task.projects.iter().filter_map(|p| project_map.get(p)).next() {
        Some(project) => Some(TimeEntry {
          id: build_time_entry_id(&self.user, &task.id, &self.date),
          date_at: self.date,
          minutes: self.time,
          customer_id: project.workspace_id.clone(),
          customer_name: project.workspace_name.clone(),
          project_id: project.id.clone(),
          project_name: project.name.clone(),
          service_id: task.id,
          service_name: task.name,
          user_id: user.id.clone(),
          user_name: user.name.clone(),
          note: self.comment,
          billable: true,
          locked: self.is_locked,
          created_at: self.created_at,
        }),
        _ => None,
      },
      _ => None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourCreateTimeRecord {
  pub date: NaiveDate,
  #[serde(with = "minutes_in_seconds")]
  pub time: Minutes,
  pub user: UserId,
  pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourUserSimple {
  pub id: UserId,
  pub name: String,
  pub email: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EverhourTimer {
  pub status: String,
  pub task: Option<EverhourTask>,
  pub user: Option<EverhourUserSimple>,
  #[serde(with = "minutes_in_seconds", default)]
  pub duration: Minutes,
  #[serde(with = "date_format", default = "default_started_at")]
  pub started_at: DateTime<Utc>,
  pub comment: Option<String>,
}

fn default_started_at() -> DateTime<Utc> {
  Utc::now()
}

pub fn day_query_param(day: &Day) -> String {
  match day {
    Day::Today => format!("{}", Local::now().naive_local().date()),
    Day::Yesterday => format!("{}", Local::now().naive_local().date().pred()),
    Day::Date(date) => format!("{}", date),
  }
}

pub fn date_span_query_param(span: &DateSpan) -> String {
  match span {
    DateSpan::ThisWeek => {
      let now = Local::now().naive_local().date();
      let year = now.year();
      let week = now.iso_week().week();

      let mon = NaiveDate::from_isoywd(year, week, Weekday::Mon);
      let sun = NaiveDate::from_isoywd(year, week, Weekday::Sun);

      format!("from={}&to={}", mon, sun)
    }
    DateSpan::LastWeek => {
      let now = Local::now().naive_local().date();

      let sun = NaiveDate::from_isoywd(now.year(), now.iso_week().week(), Weekday::Mon).succ();
      let mon = NaiveDate::from_isoywd(sun.year(), sun.iso_week().week(), Weekday::Mon);

      format!("from={}&to={}", mon, sun)
    }
    DateSpan::ThisMonth => {
      let now = Local::now().naive_local().date();
      let year = now.year();
      let month = now.month();

      let start = NaiveDate::from_ymd(year, month, 1);
      let end = if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1).pred()
      } else {
        NaiveDate::from_ymd(year, month + 1, 1).pred()
      };

      format!("from={}&to={}", start, end)
    }
    DateSpan::LastMonth => {
      let now = Local::now().naive_local().date();
      let end = NaiveDate::from_ymd(now.year(), now.month(), 1).pred();
      let year = end.year();
      let month = end.month();

      let start = if month == 1 {
        NaiveDate::from_ymd(year - 1, 12, 1)
      } else {
        NaiveDate::from_ymd(year, month - 1, 1)
      };

      format!("from={}&to={}", start, end)
    }
    DateSpan::Day(date) => format!("from={}&to={}", day_query_param(&date), day_query_param(&date)),
    DateSpan::FromTo(from, to) => format!("from={}&to={}", from, to),
  }
}

pub fn build_time_entry_id(user_id: &UserId, service_id: &ServiceId, date: &NaiveDate) -> TimeEntryId {
  TimeEntryId::Str(format!("{}|{}|{}", user_id.str_encoded(), service_id.str_encoded(), date))
}

pub fn parse_time_entry_id(time_entry_id: &TimeEntryId) -> Result<(UserId, ServiceId, NaiveDate), AcariError> {
  let parts: Vec<&str> = match time_entry_id {
    TimeEntryId::Str(s) => s.split('|').collect(),
    _ => return Err(AcariError::InternalError("Invalid time entry id (no number)".to_string())),
  };
  if parts.len() != 3 {
    return Err(AcariError::InternalError("Invalid time entry id (invalid parts)".to_string()));
  }
  let user_id = UserId::parse_encoded(&parts[0])?;
  let service_id = ServiceId::parse_encoded(&parts[1])?;
  let date = NaiveDate::parse_from_str(parts[2], "%Y-%m-%d")?;

  Ok((user_id, service_id, date))
}

mod date_format {
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};

  const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

  pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    Utc
      .datetime_from_str(&s, FORMAT)
      .or_else(|_| Utc.datetime_from_str(&format!("{} 00:00:00", &s), FORMAT))
      .map_err(serde::de::Error::custom)
  }
}

mod minutes_in_seconds {
  use crate::model::Minutes;
  use serde::{self, Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(minutes: &Minutes, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u32(minutes.0 * 60)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Minutes, D::Error>
  where
    D: Deserializer<'de>,
  {
    let seconds = u32::deserialize(deserializer)?;

    Ok(Minutes(seconds / 60))
  }
}
