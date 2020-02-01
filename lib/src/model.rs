use chrono::{DateTime, Utc};
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
  pub archived: bool,
  pub role: String,
  pub language: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MiteResponse {
  Account(Account),
  User(User),
  Error(String),
}
