mod check;
mod customers;
mod entries;
mod init;
mod projects;
mod services;

pub use check::*;
pub use customers::*;
pub use entries::*;
pub use init::*;
pub use projects::*;
pub use services::*;

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
