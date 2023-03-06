use acari_lib::{AcariError, Client};
use clap::Args;

use super::{all_projects, projects_of_customer, OutputFormat};

#[derive(Debug, Args, PartialEq, Eq)]
pub struct ProjectsCmd {
  #[clap(help = "Optional: List only projects of a specific customer")]
  customer: Option<String>,
}

impl ProjectsCmd {
  pub fn run(&self, client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
    match &self.customer {
      Some(customer) => projects_of_customer(client, output_format, customer),
      None => all_projects(client, output_format),
    }
  }
}
