use crate::error::AcariError;
use crate::model::{Account, Customer, Minutes, Project, Service, TimeEntry, Tracker, User};
use crate::model::{ProjectId, ServiceId, TimeEntryId};
use crate::query::{DateSpan, Day};
use crate::Client;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use std::{
  fs::{self, File},
  path::Path,
};

pub fn clear_cache() -> Result<(), AcariError> {
  let cache_dir = cache_dir();

  fs::remove_dir_all(cache_dir)?;

  Ok(())
}

#[derive(Debug)]
pub struct CachedClient<C> {
  client: C,
  cache_dir: PathBuf,
  cache_ttl: Duration,
}

impl<C> CachedClient<C>
where
  C: Client,
{
  pub fn new(client: C, cache_ttl: Duration) -> Result<CachedClient<C>, AcariError> {
    let cache_dir = cache_dir().join(client.get_domain());

    fs::create_dir_all(&cache_dir)?;

    Ok(CachedClient { client, cache_dir, cache_ttl })
  }

  fn cache_data<T, F>(&self, cache_name: &str, fetch_data: F) -> Result<T, AcariError>
  where
    T: DeserializeOwned + Serialize,
    F: FnOnce() -> Result<T, AcariError>,
  {
    let cache_file = self.cache_dir.join(cache_name);
    let cache_valid = file_age(&cache_file)?.map(|age| age < self.cache_ttl).unwrap_or(false);

    if cache_valid {
      Ok(serde_json::from_reader(File::open(cache_file)?)?)
    } else {
      match fetch_data() {
        Ok(data) => {
          serde_json::to_writer(File::create(cache_file)?, &data)?;
          Ok(data)
        }
        err => err,
      }
    }
  }
}

impl<C> Client for CachedClient<C>
where
  C: Client,
{
  fn get_domain(&self) -> String {
    self.client.get_domain()
  }

  fn get_account(&self) -> Result<Account, AcariError> {
    self.cache_data("account.json", || self.client.get_account())
  }

  fn get_myself(&self) -> Result<User, AcariError> {
    self.cache_data("user.json", || self.client.get_myself())
  }

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError> {
    self.cache_data("customers.json", || self.client.get_customers())
  }

  fn get_projects(&self) -> Result<Vec<Project>, AcariError> {
    self.cache_data("projects.json", || self.client.get_projects())
  }

  fn get_services(&self, project_id: &ProjectId) -> Result<Vec<Service>, AcariError> {
    self.cache_data(&format!("services-{}.json", project_id), || self.client.get_services(project_id))
  }

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError> {
    self.client.get_time_entries(date_span) // This should not be cached
  }

  fn create_time_entry(
    &self,
    day: Day,
    project_id: &ProjectId,
    service_id: &ServiceId,
    minutes: Minutes,
    note: Option<String>,
  ) -> Result<TimeEntry, AcariError> {
    self.client.create_time_entry(day, project_id, service_id, minutes, note)
  }

  fn update_time_entry(&self, entry_id: &TimeEntryId, minutes: Minutes, note: Option<String>) -> Result<(), AcariError> {
    self.client.update_time_entry(entry_id, minutes, note)
  }

  fn delete_time_entry(&self, entry_id: &TimeEntryId) -> Result<(), AcariError> {
    self.client.delete_time_entry(entry_id)
  }

  fn get_tracker(&self) -> Result<Tracker, AcariError> {
    self.client.get_tracker() // This should not be cached
  }

  fn create_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError> {
    self.client.create_tracker(entry_id)
  }

  fn delete_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError> {
    self.client.delete_tracker(entry_id)
  }
}

fn file_age(path: &Path) -> Result<Option<Duration>, AcariError> {
  match fs::metadata(path) {
    Ok(metadata) => Ok(Some(metadata.modified()?.elapsed()?)),
    Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
    Err(err) => Err(err.into()),
  }
}

fn cache_dir() -> PathBuf {
  let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
  dirs::cache_dir()
    .map(|cache| cache.join("acari"))
    .unwrap_or_else(|| home_dir.join(".acari_cache"))
}
