use crate::error::AcariError;
use crate::user_error;
use chrono::{DateTime, NaiveDate, Utc};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops;
use std::str::FromStr;

macro_rules! id_wrapper {
  ($name: ident) => {
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    pub enum $name {
      Num(u64),
      Str(String),
    }

    impl $name {
      pub fn str_encoded(&self) -> String {
        match self {
          $name::Num(n) => format!("n{}", n),
          $name::Str(s) => format!("s{}", s),
        }
      }

      pub fn parse_encoded(s: &str) -> Result<$name, AcariError> {
        match s.chars().next() {
          Some('n') => Ok($name::Num(s[1..].parse::<u64>()?)),
          Some('s') => Ok($name::Str(s[1..].to_string())),
          _ => Err(AcariError::InternalError("Invalid id format".to_string())),
        }
      }

      pub fn path_encoded(&self) -> String {
        match self {
          $name::Num(n) => n.to_string(),
          $name::Str(s) => utf8_percent_encode(&s, NON_ALPHANUMERIC).to_string(),
        }
      }
    }

    impl Default for $name {
      fn default() -> Self {
        $name::Num(0)
      }
    }

    impl Serialize for $name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: Serializer,
      {
        match self {
          $name::Num(n) => serializer.serialize_u64(*n),
          $name::Str(s) => serializer.serialize_str(s),
        }
      }
    }

    impl<'de> Deserialize<'de> for $name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: Deserializer<'de>,
      {
        struct EnumVisitor;

        impl<'de> Visitor<'de> for EnumVisitor {
          type Value = $name;

          fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("integer or string")
          }

          fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
          where
            E: Error,
          {
            Ok($name::Num(v))
          }

          fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
          where
            E: Error,
          {
            Ok($name::Str(v.to_string()))
          }
        }

        deserializer.deserialize_any(EnumVisitor)
      }
    }

    impl fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          $name::Num(n) => write!(f, "{}", n),
          $name::Str(s) => write!(f, "{}", s),
        }
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Customer {
  pub id: CustomerId,
  pub name: String,
  pub note: String,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Project {
  pub id: ProjectId,
  pub name: String,
  pub customer_id: CustomerId,
  pub customer_name: String,
  pub note: String,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Service {
  pub id: ServiceId,
  pub name: String,
  pub note: String,
  pub billable: bool,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
  pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tracker {
  pub since: Option<DateTime<Utc>>,
  pub tracking_time_entry: Option<TimeEntry>,
  pub stopped_time_entry: Option<TimeEntry>,
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
