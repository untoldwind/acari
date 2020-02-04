use crate::error::AcariError;
use chrono::{Local, NaiveDate};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Day {
  Today,
  Yesterday,
  Date(NaiveDate),
}

impl Day {
  pub fn query_param(self) -> String {
    match self {
      Day::Today => "today".to_string(),
      Day::Yesterday => "yesterday".to_string(),
      Day::Date(date) => format!("{}", date),
    }
  }

  pub fn as_date(self) -> NaiveDate {
    match self {
      Day::Today => Local::now().naive_local().date(),
      Day::Yesterday => Local::now().naive_local().date().pred(),
      Day::Date(date) => date,
    }
  }
}

impl FromStr for Day {
  type Err = AcariError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "today" | "now" => Ok(Day::Today),
      "yesterday" => Ok(Day::Yesterday),
      date => Ok(Day::Date(NaiveDate::parse_from_str(date, "%Y-%m-%d")?)),
    }
  }
}

impl From<NaiveDate> for Day {
  fn from(date: NaiveDate) -> Self {
    Day::Date(date)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum DateSpan {
  ThisWeek,
  LastWeek,
  ThisMonth,
  LastMonth,
  Day(Day),
  FromTo(NaiveDate, NaiveDate),
}

impl DateSpan {
  pub fn query_param(&self) -> String {
    match self {
      DateSpan::ThisWeek => "at=this_week".to_string(),
      DateSpan::LastWeek => "at=last_week".to_string(),
      DateSpan::ThisMonth => "at=this_month".to_string(),
      DateSpan::LastMonth => "at=last_month".to_string(),
      DateSpan::Day(date) => format!("at={}", date.query_param()),
      DateSpan::FromTo(from, to) => format!("from={}&to={}", from, to),
    }
  }
}

impl FromStr for DateSpan {
  type Err = AcariError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "this-week" | "week" => Ok(DateSpan::ThisWeek),
      "last-week" => Ok(DateSpan::LastWeek),
      "this-month" | "month" => Ok(DateSpan::ThisMonth),
      "last-month" => Ok(DateSpan::LastMonth),
      date_or_range => match date_or_range.find('/') {
        Some(idx) => Ok(DateSpan::FromTo(
          NaiveDate::parse_from_str(&date_or_range[..idx], "%Y-%m-%d")?,
          NaiveDate::parse_from_str(&date_or_range[idx + 1..], "%Y-%m-%d")?,
        )),
        None => Ok(DateSpan::Day(date_or_range.parse()?)),
      },
    }
  }
}

impl From<Day> for DateSpan {
  fn from(day: Day) -> Self {
    DateSpan::Day(day)
  }
}

impl From<NaiveDate> for DateSpan {
  fn from(date: NaiveDate) -> Self {
    DateSpan::Day(Day::Date(date))
  }
}
