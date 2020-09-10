use crate::error::AcariError;
use crate::user_error;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;
use std::str::FromStr;

macro_rules! id_wrapper {
  ($name: ident) => {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
    #[serde(transparent)]
    pub struct $name(pub u32);

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Account {
  pub id: AccountId,
  pub name: String,
  pub title: String,
  pub currency: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Customer {
  pub id: CustomerId,
  pub name: String,
  pub note: String,
  pub hourly_rate: Option<u32>,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Project {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Minutes(pub u32);

impl fmt::Display for Minutes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:{:02}", self.0 / 60, self.0 % 60)
  }
}

impl ops::Add for Minutes {
  type Output = Minutes;
  fn add(self, rhs: Minutes) -> Self::Output {
    Minutes(self.0 + rhs.0)
  }
}

impl ops::AddAssign for Minutes {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
  }
}

impl std::iter::Sum<Minutes> for Minutes {
  fn sum<I: Iterator<Item = Minutes>>(iter: I) -> Self {
    Minutes(iter.map(|m| m.0).sum())
  }
}

impl FromStr for Minutes {
  type Err = AcariError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.find(':') {
      Some(idx) => {
        let hours = s[..idx].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;
        let minutes = s[idx + 1..].parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?;

        if minutes >= 60 {
          Err(AcariError::UserError("No more than 60 minutes per hour".to_string()))
        } else if hours >= 24 {
          Err(AcariError::UserError("No more than 24 hour per day".to_string()))
        } else {
          Ok(Minutes(hours * 60 + minutes))
        }
      }
      None => Ok(Minutes(s.parse::<u32>().map_err(|e| user_error!("Invalid time format: {}", e))?)),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackingTimeEntry {
  pub id: TimeEntryId,
  pub minutes: Minutes,
  pub since: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_parse_minutes() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("123".parse::<Minutes>()?, Minutes(123));
    assert_eq!("0:40".parse::<Minutes>()?, Minutes(40));
    assert_eq!("5:35".parse::<Minutes>()?, Minutes(5 * 60 + 35));

    Ok(())
  }
}
