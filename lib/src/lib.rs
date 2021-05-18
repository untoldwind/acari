mod cached_client;
mod error;
mod everhour_client;
mod everhour_model;
mod mite_client;
mod mite_model;
mod model;
mod query;
mod requester;

pub use cached_client::{clear_cache, CachedClient};
pub use error::AcariError;
pub use everhour_client::EverhourClient;
pub use mite_client::MiteClient;
pub use model::{Account, Customer, Minutes, Project, Service, TimeEntry, Tracker, User};
pub use model::{AccountId, CustomerId, ProjectId, ServiceId, TimeEntryId, UserId};
pub use query::{DateSpan, Day};

#[cfg(test)]
mod mite_client_tests;

#[cfg(test)]
mod everhour_client_tests;

pub trait Client {
  fn get_domain(&self) -> String;

  fn get_account(&self) -> Result<Account, AcariError>;

  fn get_myself(&self) -> Result<User, AcariError>;

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError>;

  fn get_projects(&self) -> Result<Vec<Project>, AcariError>;

  fn get_services(&self, project_id: &ProjectId) -> Result<Vec<Service>, AcariError>;

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError>;

  fn create_time_entry(
    &self,
    day: Day,
    project_id: &ProjectId,
    service_id: &ServiceId,
    minutes: Minutes,
    note: Option<String>,
  ) -> Result<TimeEntry, AcariError>;

  fn update_time_entry(&self, entry_id: &TimeEntryId, minutes: Minutes, note: Option<String>) -> Result<(), AcariError>;

  fn delete_time_entry(&self, entry_id: &TimeEntryId) -> Result<(), AcariError>;

  fn get_tracker(&self) -> Result<Tracker, AcariError>;

  fn create_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError>;

  fn delete_tracker(&self, entry_id: &TimeEntryId) -> Result<Tracker, AcariError>;
}

#[macro_export]
macro_rules! user_error {
  ( $( $arg:expr ),* ) => {
    AcariError::UserError(format!($($arg),*))
  }
}

#[macro_export]
macro_rules! internal_error {
  ( $( $arg:expr ),* ) => {
    AcariError::InternalError(format!($($arg),*))
  }
}
