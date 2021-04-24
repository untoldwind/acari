use crate::everhour_model::{
  build_time_entry_id, date_span_query_param, parse_time_entry_id, EverhourCreateTimeRecord, EverhourError, EverhourTask, EverhourTimeEntry, EverhourTimer,
  EverhourUser,
};
use crate::model::{Account, Customer, CustomerId, Minutes, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, Tracker, User};
use crate::query::{DateSpan, Day};
use crate::Client;
use crate::{error::AcariError, everhour_model::EverhourProject};
use chrono::Utc;
use reqwest::{blocking, header, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::json;
use std::collections::HashMap;
use url::Url;

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

#[derive(Debug)]
pub struct EverhourClient {
  base_url: Url,
  client: blocking::Client,
}

impl EverhourClient {
  pub fn new(domain: &str, token: &str) -> Result<EverhourClient, AcariError> {
    Ok(Self::new_form_url(format!("https://{}@{}", token, domain).parse()?))
  }

  pub fn new_form_url(base_url: Url) -> EverhourClient {
    EverhourClient {
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
        .header("X-Api-Key", self.base_url.username()),
    )
  }

  fn request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    Self::handle_response(response)
  }

  fn request_with_body<T: DeserializeOwned, D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    Self::handle_response(response)
  }

  fn handle_response<T: DeserializeOwned>(response: blocking::Response) -> Result<T, AcariError> {
    match response.status() {
      StatusCode::OK | StatusCode::CREATED => Ok(response.json()?),
      status => match response.json::<EverhourError>() {
        Ok(err) => Err(AcariError::Mite(err.code, err.message)),
        _ => Err(AcariError::Mite(status.as_u16(), status.to_string())),
      },
    }
  }

  fn entry_from_timer(&self, timer: EverhourTimer) -> Result<Option<TimeEntry>, AcariError> {
    match (timer.status.as_str(), timer.task, timer.user) {
      ("active", Some(task), Some(user)) => {
        let maybe_project = match task.projects.get(0) {
          Some(project_id) => Some(self.request::<EverhourProject>(Method::GET, &format!("/projects/{}", project_id.path_encoded()))?),
          None => None,
        };
        Ok(Some(TimeEntry {
          id: build_time_entry_id(&user.id, &task.id, &timer.started_at.naive_utc().date()),
          date_at: timer.started_at.naive_utc().date(),
          minutes: timer.duration,
          customer_id: maybe_project.as_ref().map(|p| p.workspace_id.clone()).unwrap_or_default(),
          customer_name: maybe_project.as_ref().map(|p| p.workspace_name.clone()).unwrap_or_default(),
          project_id: maybe_project.as_ref().map(|p| p.id.clone()).unwrap_or_default(),
          project_name: maybe_project.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
          service_id: task.id,
          service_name: task.name,
          user_id: user.id.clone(),
          user_name: user.name.clone(),
          note: timer.comment.unwrap_or_default(),
          billable: true,
          locked: false,
          created_at: timer.started_at,
        }))
      }
      _ => Ok(None),
    }
  }
}

impl Client for EverhourClient {
  fn get_domain(&self) -> String {
    self.base_url.host_str().unwrap_or("").to_owned()
  }

  fn get_account(&self) -> Result<Account, AcariError> {
    Ok(self.request::<EverhourUser>(Method::GET, "/users/me")?.into())
  }

  fn get_myself(&self) -> Result<User, AcariError> {
    Ok(self.request::<EverhourUser>(Method::GET, "/users/me")?.into())
  }

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError> {
    let projects = self.request::<Vec<EverhourProject>>(Method::GET, "/projects")?;
    let mut customers_map: HashMap<CustomerId, Customer> = HashMap::new();

    for project in projects {
      let created_at = project.created_at;
      let archived = project.status != "open";
      let customer_ref = customers_map.entry(project.workspace_id.clone()).or_insert_with(|| project.into());

      if created_at < customer_ref.created_at {
        customer_ref.created_at = created_at;
      }
      if !archived {
        customer_ref.archived = false;
      }
    }

    Ok(customers_map.into_iter().map(|(_, v)| v).collect())
  }

  fn get_projects(&self) -> Result<Vec<Project>, AcariError> {
    let projects = self.request::<Vec<EverhourProject>>(Method::GET, "/projects")?;

    Ok(projects.into_iter().map(Into::into).collect())
  }

  fn get_services(&self, project_id: &ProjectId) -> Result<Vec<Service>, AcariError> {
    let tasks = self.request::<Vec<EverhourTask>>(Method::GET, &format!("/projects/{}/tasks", project_id.path_encoded()))?;

    Ok(tasks.into_iter().map(Into::into).collect())
  }

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError> {
    let user = self.request::<EverhourUser>(Method::GET, "/users/me")?;
    let project_map: HashMap<ProjectId, EverhourProject> = self
      .request::<Vec<EverhourProject>>(Method::GET, "/projects")?
      .into_iter()
      .map(|p| (p.id.clone(), p))
      .collect();
    let entries = self.request::<Vec<EverhourTimeEntry>>(Method::GET, &format!("/users/me/time?{}", date_span_query_param(&date_span)))?;

    Ok(entries.into_iter().filter_map(|e| e.into_entry(&project_map, &user)).collect())
  }

  fn create_time_entry(&self, day: Day, _: &ProjectId, service_id: &ServiceId, minutes: Minutes, note: Option<String>) -> Result<TimeEntry, AcariError> {
    let user = self.request::<EverhourUser>(Method::GET, "/users/me")?;
    let project_map: HashMap<ProjectId, EverhourProject> = self
      .request::<Vec<EverhourProject>>(Method::GET, "/projects")?
      .into_iter()
      .map(|p| (p.id.clone(), p))
      .collect();

    let entry: EverhourTimeEntry = self.request_with_body(
      Method::POST,
      &format!("/tasks/{}/time", service_id.path_encoded()),
      EverhourCreateTimeRecord {
        date: day.as_date(),
        user: user.id.clone(),
        time: minutes,
        comment: note,
      },
    )?;

    entry
      .into_entry(&project_map, &user)
      .ok_or_else(|| AcariError::InternalError("Invalid time entry id (invalid parts)".to_string()))
  }

  fn update_time_entry(&self, entry_id: &TimeEntryId, minutes: Minutes, note: Option<String>) -> Result<(), AcariError> {
    let (user_id, service_id, date) = parse_time_entry_id(entry_id)?;

    let _: EverhourTimeEntry = self.request_with_body(
      Method::POST,
      &format!("/tasks/{}/time", service_id.path_encoded()),
      EverhourCreateTimeRecord {
        date,
        user: user_id,
        time: minutes,
        comment: note,
      },
    )?;

    Ok(())
  }

  fn delete_time_entry(&self, entry_id: &TimeEntryId) -> Result<(), AcariError> {
    let (user_id, service_id, date) = parse_time_entry_id(entry_id)?;

    let _: EverhourTimeEntry = self.request_with_body(
      Method::POST,
      &format!("/tasks/{}/time", service_id.path_encoded()),
      json!({
        "user": user_id,
        "date": date,
      }),
    )?;

    Ok(())
  }

  fn get_tracker(&self) -> Result<Tracker, AcariError> {
    let timer = self.request::<EverhourTimer>(Method::GET, "/timers/current")?;
    let started_at = timer.started_at;

    match self.entry_from_timer(timer)? {
      Some(time_entry) => Ok(Tracker {
        since: Some(started_at),
        tracking_time_entry: Some(time_entry),
        stopped_time_entry: None,
      }),
      _ => Ok(Tracker {
        since: None,
        tracking_time_entry: None,
        stopped_time_entry: None,
      }),
    }
  }

  fn create_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError> {
    let (_, service_id, date) = parse_time_entry_id(entry_id)?;
    let timer: EverhourTimer = self.request_with_body(
      Method::POST,
      "/timers",
      json!({
        "task": service_id,
        "userDate": date,
      }),
    )?;

    Ok(Tracker {
      since: Some(Utc::now()),
      tracking_time_entry: self.entry_from_timer(timer)?,
      stopped_time_entry: None,
    })
  }

  fn delete_tracker(&self, _: &TimeEntryId) -> Result<Tracker, AcariError> {
    let timer = self.request::<EverhourTimer>(Method::DELETE, "/timers/current")?;

    Ok(Tracker {
      since: None,
      tracking_time_entry: None,
      stopped_time_entry: self.entry_from_timer(timer)?,
    })
  }
}
