use clap::Clap;

mod add;
mod all_projects;
mod check;
mod customers;
mod entries;
mod init;
mod profiles;
mod projects;
mod projects_of_customer;
mod services;
mod set;
mod tracker;

pub use add::*;
pub use all_projects::*;
pub use check::*;
pub use customers::*;
pub use entries::*;
pub use init::*;
pub use profiles::*;
pub use projects::*;
pub use projects_of_customer::*;
pub use services::*;
pub use set::*;
pub use tracker::*;

use acari_lib::{user_error, AcariError, Client, Customer, CustomerId, Project, ProjectId, Service};

#[derive(Clap, Debug, PartialEq)]
pub enum OutputFormat {
  Pretty,
  Json,
  Flat,
}

fn find_customer(client: &dyn Client, customer_name: &str) -> Result<Customer, AcariError> {
  let customers = client.get_customers()?;

  customers
    .into_iter()
    .find(|c| c.name == customer_name)
    .ok_or_else(|| user_error!("No customer with name: {}", customer_name))
}

fn find_project(client: &dyn Client, customer_id: &CustomerId, project_name: &str) -> Result<Project, AcariError> {
  let projects = client.get_projects()?;

  projects
    .into_iter()
    .find(|p| p.name == project_name && p.customer_id.eq(customer_id))
    .ok_or_else(|| user_error!("No project with name: {}", project_name))
}

fn find_service(client: &dyn Client, project_id: &ProjectId, service_name: &str) -> Result<Service, AcariError> {
  let services = client.get_services(project_id)?;

  services
    .into_iter()
    .find(|s| s.name == service_name)
    .ok_or_else(|| user_error!("No service with name: {}", service_name))
}
