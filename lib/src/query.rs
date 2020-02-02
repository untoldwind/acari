use crate::error::AcariError;
use chrono::NaiveDate;

#[derive(Debug)]
pub enum Day {
  Today,
  Yesterday,
  Date(NaiveDate),
}

impl Day {
  pub fn from_string(day: &str) -> Result<Day, AcariError> {
    match day.to_lowercase().as_str() {
      "today" | "now" => Ok(Day::Today),
      "yesterday" => Ok(Day::Yesterday),
      date => Ok(Day::Date(NaiveDate::parse_from_str(date, "%Y-%m-%d")?)),
    }
  }

  pub fn query_param(&self) -> String {
    match self {
      Day::Today => "today".to_string(),
      Day::Yesterday => "yesterday".to_string(),
      Day::Date(date) => format!("{}", date),
    }
  }
}

#[derive(Debug)]
pub enum DateSpan {
  Today,
  Yesterday,
  ThisWeek,
  LastWeek,
  ThisMonth,
  LastMonth,
  Day(NaiveDate),
  FromTo(NaiveDate, NaiveDate),
}

impl DateSpan {
  pub fn from_string(span: &str) -> Result<DateSpan, AcariError> {
    match span.to_lowercase().as_str() {
      "today" | "now" => Ok(DateSpan::Today),
      "yesterday" => Ok(DateSpan::Yesterday),
      "this-week" | "week" => Ok(DateSpan::ThisWeek),
      "last-week" => Ok(DateSpan::LastWeek),
      "this-month" | "month" => Ok(DateSpan::ThisMonth),
      "last-month" => Ok(DateSpan::LastMonth),
      date_or_range => match date_or_range.find("|") {
        Some(idx) => Ok(DateSpan::FromTo(
          NaiveDate::parse_from_str(&date_or_range[..idx], "%Y-%m-%d")?,
          NaiveDate::parse_from_str(&date_or_range[idx + 1..], "%Y-%m-%d")?,
        )),
        None => Ok(DateSpan::Day(NaiveDate::parse_from_str(date_or_range, "%Y-%m-%d")?)),
      },
    }
  }

  pub fn query_param(&self) -> String {
    match self {
      DateSpan::Today => "at=today".to_string(),
      DateSpan::Yesterday => "at=yesterday".to_string(),
      DateSpan::ThisWeek => "at=this_week".to_string(),
      DateSpan::LastWeek => "at=last_week".to_string(),
      DateSpan::ThisMonth => "at=this_month".to_string(),
      DateSpan::LastMonth => "at=last_month".to_string(),
      DateSpan::Day(date) => format!("at={}", date),
      DateSpan::FromTo(from, to) => format!("from={}&to={}", from, to),
    }
  }
}
