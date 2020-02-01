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
#[serde(rename_all = "lowercase")]
pub enum MiteEntity {
  Account(Account),
  User(User),
  Customer(Customer),
  Project(Project),
  Service(Service),
  Error(String),
}
