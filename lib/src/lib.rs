mod cached_client;
mod error;
mod model;
mod query;
mod std_client;

pub use cached_client::CachedClient;
pub use error::AcariError;
pub use model::{
  Account, AccountId, Customer, CustomerId, Minutes, Project, ProjectId, Service, ServiceId, TimeEntry, TimeEntryId, Tracker, TrackingTimeEntry, User, UserId,
};
pub use query::{DateSpan, Day};
pub use std_client::StdClient;

pub trait Client {
  fn get_account(&self) -> Result<Account, AcariError>;

  fn get_myself(&self) -> Result<User, AcariError>;

  fn get_customers(&self) -> Result<Vec<Customer>, AcariError>;

  fn get_projects(&self) -> Result<Vec<Project>, AcariError>;

  fn get_services(&self) -> Result<Vec<Service>, AcariError>;

  fn get_time_entry(&self, entry_id: TimeEntryId) -> Result<TimeEntry, AcariError>;

  fn get_time_entries(&self, date_span: DateSpan) -> Result<Vec<TimeEntry>, AcariError>;

  fn create_time_entry(&self, day: Day, project_id: ProjectId, service_id: ServiceId, minutes: Minutes) -> Result<TimeEntry, AcariError>;

  fn update_time_entry(&self, entry_id: TimeEntryId, minutes: Minutes) -> Result<(), AcariError>;

  fn delete_time_entry(&self, entry_id: TimeEntryId) -> Result<(), AcariError>;

  fn get_tracker(&self) -> Result<Tracker, AcariError>;

  fn create_tracker(&self, entry_id: TimeEntryId) -> Result<Tracker, AcariError>;

  fn delete_tracker(&self, entry_id: TimeEntryId) -> Result<Tracker, AcariError>;
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
