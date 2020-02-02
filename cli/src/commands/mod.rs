mod all_projects;
mod check;
mod clear_cache;
mod customers;
mod entries;
mod init;
mod projects_of_customer;
mod services;
mod set;
mod start;
mod stop;
mod tracking;

pub use all_projects::*;
pub use check::*;
pub use clear_cache::*;
pub use customers::*;
pub use entries::*;
pub use init::*;
pub use projects_of_customer::*;
pub use services::*;
pub use set::*;
pub use start::*;
pub use stop::*;
pub use tracking::*;

use acari_lib::{user_error, AcariError, Client, Customer, CustomerId, Project, Service};

pub enum OutputFormat {
  Pretty,
  Json,
  Flat,
}

impl OutputFormat {
  pub fn from_string(format: &str) -> Result<OutputFormat, AcariError> {
    match format {
      "pretty" => Ok(OutputFormat::Pretty),
      "json" => Ok(OutputFormat::Json),
      "flat" => Ok(OutputFormat::Flat),
      format => Err(AcariError::UserError(format!("Invalid output format: {}", format))),
    }
  }
}

fn find_customer(client: &dyn Client, customer_name: &str) -> Result<Customer, AcariError> {
  let customers = client.get_customers()?;

  customers
    .into_iter()
    .find(|c| c.name == customer_name)
    .ok_or_else(|| user_error!("No customer with name: {}", customer_name))
}

fn find_project(client: &dyn Client, customer_id: CustomerId, project_name: &str) -> Result<Project, AcariError> {
  let projects = client.get_projects()?;

  projects
    .into_iter()
    .find(|p| p.name == project_name && p.customer_id == customer_id)
    .ok_or_else(|| user_error!("No project with name: {}", project_name))
}

fn find_service(client: &dyn Client, service_name: &str) -> Result<Service, AcariError> {
  let services = client.get_services()?;

  services
    .into_iter()
    .find(|s| s.name == service_name)
    .ok_or_else(|| user_error!("No service with name: {}", service_name))
}
