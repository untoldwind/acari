use crate::model::{MiteEntity, Account, Project, Customer, TimeEntry, User, Service};
use crate::query::DateSpan;
use crate::error::AcariError;
use serde::de::DeserializeOwned;

use reqwest::{blocking, header, StatusCode};

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

#[derive(Debug)]
pub struct Client {
  domain: String,
  token: String,
  client: blocking::Client,
}

impl Client {
  pub fn new(domain: &str, token: &str) -> Client {
    Client {
      domain: domain.to_string(),
      token: token.to_string(),
      client: blocking::Client::new(),
    }
  }

  fn get<T: DeserializeOwned>(&self, uri: &str) -> Result<T, AcariError> {
    let response = self
      .client
      .get(&format!("https://{}{}", self.domain, uri))
      .header(header::USER_AGENT, USER_AGENT)
      .header(header::HOST, &self.domain)
      .header("X-MiteApiKey", &self.token)
      .send()?;

    handle_response(response)
  }

  pub fn get_account(&self) -> Result<Account, AcariError> {
    match self.get("/account.json")? {
      MiteEntity::Account(account) => Ok(account),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  pub fn get_myself(&self) -> Result<User, AcariError> {
    match self.get("/myself.json")? {
      MiteEntity::User(user) => Ok(user),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  pub fn get_customers(&self) -> Result<Vec<Customer>, AcariError> {
    Ok(
      self
        .get::<Vec<MiteEntity>>("/customers.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Customer(customer) => Some(customer),
          _ => None,
        })
        .collect(),
    )
  }

  pub fn get_projects(&self) -> Result<Vec<Project>, AcariError> {
    Ok(
      self
        .get::<Vec<MiteEntity>>("/projects.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Project(project) => Some(project),
          _ => None,
        })
        .collect(),
    )
  }

  pub fn get_services(&self) -> Result<Vec<Service>, AcariError> {
    Ok(
      self
        .get::<Vec<MiteEntity>>("/services.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Service(service) => Some(service),
          _ => None,
        })
        .collect(),
    )
  }

  pub fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError> {
    Ok(
      self
        .get::<Vec<MiteEntity>>(&format!("/time_entries.json?user=current&{}", date_span.query_param()))?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::TimeEntry(time_entry) => Some(time_entry),
          _ => None,
        })
        .collect(),
    )
  }
}

fn handle_response<T: DeserializeOwned>(response: blocking::Response) -> Result<T, AcariError> {
  match response.status() {
    StatusCode::OK => Ok(response.json()?),
    status => match response.json::<MiteEntity>() {
      Ok(MiteEntity::Error(msg)) => Err(AcariError::Mite(status.as_u16(), msg)),
      _ => Err(AcariError::Mite(status.as_u16(), status.to_string())),
    },
  }
}
