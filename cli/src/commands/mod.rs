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

use acari_lib::AcariError;

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
