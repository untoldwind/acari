use crate::error::AcariError;
use crate::model::{Account, Customer, Minutes, MiteEntity, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, Tracker, User};
use crate::query::{DateSpan, Day};
use crate::Client;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::json;
use url::Url;

use reqwest::{blocking, header, StatusCode};

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

#[derive(Debug)]
pub struct StdClient {
  base_url: Url,
  client: blocking::Client,
}

impl StdClient {
  pub fn new(domain: &str, token: &str) -> Result<StdClient, AcariError> {
    Ok(Self::new_form_url(format!("https://{}@{}", token, domain).parse()?))
  }

  pub fn new_form_url(base_url: Url) -> StdClient {
    StdClient {
      base_url,
      client: blocking::Client::new(),
    }
  }

  fn base_request(&self, method: Method, uri: &str) -> Result<blocking::RequestBuilder, AcariError> {
    Ok(
      self
        .client
        .request(method, self.base_url.join(uri)?.as_str())
        .header(header::USER_AGENT, USER_AGENT)
        .header(header::HOST, self.base_url.host_str().unwrap_or(""))
        .header("X-MiteApiKey", self.base_url.username()),
    )
  }

  fn request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    handle_response(response)
  }

  fn request_empty(&self, method: Method, uri: &str) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    handle_empty_response(response)
  }

  fn request_with_body<T: DeserializeOwned, D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    handle_response(response)
  }

  fn request_empty_with_body<D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    handle_empty_response(response)
  }
}

impl Client for StdClient {
  fn get_account(&self) -> Result<Account, AcariError> {
    match self.request(Method::GET, "/account.json")? {
      MiteEntity::Account(account) => Ok(account),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn get_myself(&self) -> Result<User, AcariError> {
    match self.request(Method::GET, "/myself.json")? {
      MiteEntity::User(user) => Ok(user),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, "/customers.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Customer(customer) => Some(customer),
          _ => None,
        })
        .collect(),
    )
  }

  fn get_projects(&self) -> Result<Vec<Project>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, "/projects.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Project(project) => Some(project),
          _ => None,
        })
        .collect(),
    )
  }

  fn get_services(&self) -> Result<Vec<Service>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, "/services.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Service(service) => Some(service),
          _ => None,
        })
        .collect(),
    )
  }

  fn get_time_entry(&self, entry_id: TimeEntryId) -> Result<TimeEntry, AcariError> {
    match self.request(Method::GET, &format!("/time_entries/{}.json", entry_id))? {
      MiteEntity::TimeEntry(time_entry) => Ok(time_entry),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, &format!("/time_entries.json?user=current&{}", date_span.query_param()))?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::TimeEntry(time_entry) => Some(time_entry),
          _ => None,
        })
        .collect(),
    )
  }

  fn create_time_entry(&self, day: Day, project_id: ProjectId, service_id: ServiceId, minutes: Minutes, note: Option<String>) -> Result<TimeEntry, AcariError> {
    match self.request_with_body(
      Method::POST,
      "/time_entries.json",
      json!({
        "time_entry": {
          "date_at": day.as_date(),
          "project_id": project_id,
          "service_id": service_id,
          "minutes": minutes,
          "note": note.unwrap_or_else(|| "".to_string()),
        }
      }),
    )? {
      MiteEntity::TimeEntry(time_entry) => Ok(time_entry),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn update_time_entry(&self, entry_id: TimeEntryId, minutes: Minutes, note: Option<String>) -> Result<(), AcariError> {
    self.request_empty_with_body(
      Method::PATCH,
      &format!("/time_entries/{}.json", entry_id),
      json!({
        "time_entry": {
          "minutes": minutes,
          "note": note.unwrap_or_else(|| "".to_string()),
        }
      }),
    )
  }

  fn delete_time_entry(&self, entry_id: TimeEntryId) -> Result<(), AcariError> {
    self.request_empty(Method::DELETE, &format!("/time_entries/{}.json", entry_id))
  }

  fn get_tracker(&self) -> Result<Tracker, AcariError> {
    match self.request(Method::GET, "/tracker.json")? {
      MiteEntity::Tracker(tracker) => Ok(tracker),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn create_tracker(&self, entry_id: TimeEntryId) -> Result<Tracker, AcariError> {
    match self.request(Method::PATCH, &format!("/tracker/{}.json", entry_id))? {
      MiteEntity::Tracker(tracker) => Ok(tracker),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn delete_tracker(&self, entry_id: TimeEntryId) -> Result<Tracker, AcariError> {
    match self.request(Method::DELETE, &format!("/tracker/{}.json", entry_id))? {
      MiteEntity::Tracker(tracker) => Ok(tracker),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }
}

fn handle_empty_response(response: blocking::Response) -> Result<(), AcariError> {
  match response.status() {
    StatusCode::OK | StatusCode::CREATED => Ok(()),
    status => match response.json::<MiteEntity>() {
      Ok(MiteEntity::Error(msg)) => Err(AcariError::Mite(status.as_u16(), msg)),
      _ => Err(AcariError::Mite(status.as_u16(), status.to_string())),
    },
  }
}

fn handle_response<T: DeserializeOwned>(response: blocking::Response) -> Result<T, AcariError> {
  match response.status() {
    StatusCode::OK | StatusCode::CREATED => Ok(response.json()?),
    status => match response.json::<MiteEntity>() {
      Ok(MiteEntity::Error(msg)) => Err(AcariError::Mite(status.as_u16(), msg)),
      _ => Err(AcariError::Mite(status.as_u16(), status.to_string())),
    },
  }
}
