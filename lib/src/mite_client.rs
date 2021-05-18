use crate::mite_model::{date_span_query_param, MiteEntity, MiteTracker};
use crate::model::{Account, Customer, Minutes, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, Tracker, User};
use crate::query::{DateSpan, Day};
use crate::Client;
use crate::{error::AcariError, requester::ResponseHandler};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::json;
use url::Url;

use reqwest::{blocking, header, Method, StatusCode};

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

#[derive(Debug)]
pub struct MiteClient {
  base_url: Url,
  client: blocking::Client,
}

impl MiteClient {
  pub fn new(domain: &str, token: &str) -> Result<MiteClient, AcariError> {
    Ok(Self::new_form_url(format!("https://{}@{}", token, domain).parse()?))
  }

  pub fn new_form_url(base_url: Url) -> MiteClient {
    MiteClient {
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

    Self::handle_response(response)
  }

  fn request_empty(&self, method: Method, uri: &str) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    Self::handle_empty_response(response)
  }

  fn request_with_body<T: DeserializeOwned, D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    Self::handle_response(response)
  }

  fn request_empty_with_body<D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    Self::handle_empty_response(response)
  }

  fn get_time_entry(&self, entry_id: &TimeEntryId) -> Result<TimeEntry, AcariError> {
    match self.request(Method::GET, &format!("/time_entries/{}.json", entry_id))? {
      MiteEntity::TimeEntry(time_entry) => Ok(time_entry.into()),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn convert_tracker(&self, tracker: MiteTracker) -> Result<Tracker, AcariError> {
    let tracking_time_entry = tracker
      .tracking_time_entry
      .as_ref()
      .map(|e| {
        self.get_time_entry(&e.id).map(|mut entry| {
          entry.minutes = e.minutes;
          entry
        })
      })
      .transpose()?;
    let stopped_time_entry = tracker.stopped_time_entry.as_ref().map(|e| self.get_time_entry(&e.id)).transpose()?;

    Ok(Tracker {
      since: tracker.tracking_time_entry.and_then(|e| e.since),
      tracking_time_entry,
      stopped_time_entry,
    })
  }
}

impl ResponseHandler for MiteClient {
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
}

impl Client for MiteClient {
  fn get_domain(&self) -> String {
    self.base_url.host_str().unwrap_or("").to_owned()
  }

  fn get_account(&self) -> Result<Account, AcariError> {
    match self.request(Method::GET, "/account.json")? {
      MiteEntity::Account(account) => Ok(account.into()),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn get_myself(&self) -> Result<User, AcariError> {
    match self.request(Method::GET, "/myself.json")? {
      MiteEntity::User(user) => Ok(user.into()),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, "/customers.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Customer(customer) => Some(customer.into()),
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
          MiteEntity::Project(project) => Some(project.into()),
          _ => None,
        })
        .collect(),
    )
  }

  fn get_services(&self, _: &ProjectId) -> Result<Vec<Service>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, "/services.json")?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::Service(service) => Some(service.into()),
          _ => None,
        })
        .collect(),
    )
  }

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError> {
    Ok(
      self
        .request::<Vec<MiteEntity>>(Method::GET, &format!("/time_entries.json?user=current&{}", date_span_query_param(&date_span)))?
        .into_iter()
        .filter_map(|entity| match entity {
          MiteEntity::TimeEntry(time_entry) => Some(time_entry.into()),
          _ => None,
        })
        .collect(),
    )
  }

  fn create_time_entry(
    &self,
    day: Day,
    project_id: &ProjectId,
    service_id: &ServiceId,
    minutes: Minutes,
    note: Option<String>,
  ) -> Result<TimeEntry, AcariError> {
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
      MiteEntity::TimeEntry(time_entry) => Ok(time_entry.into()),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn update_time_entry(&self, entry_id: &TimeEntryId, minutes: Minutes, note: Option<String>) -> Result<(), AcariError> {
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

  fn delete_time_entry(&self, entry_id: &TimeEntryId) -> Result<(), AcariError> {
    self.request_empty(Method::DELETE, &format!("/time_entries/{}.json", entry_id))
  }

  fn get_tracker(&self) -> Result<Tracker, AcariError> {
    match self.request(Method::GET, "/tracker.json")? {
      MiteEntity::Tracker(tracker) => Ok(self.convert_tracker(tracker)?),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn create_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError> {
    match self.request(Method::PATCH, &format!("/tracker/{}.json", entry_id))? {
      MiteEntity::Tracker(tracker) => Ok(self.convert_tracker(tracker)?),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  fn delete_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError> {
    match self.request(Method::DELETE, &format!("/tracker/{}.json", entry_id))? {
      MiteEntity::Tracker(tracker) => Ok(self.convert_tracker(tracker)?),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }
}
