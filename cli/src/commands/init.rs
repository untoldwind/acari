use crate::config::Config;
use acari_lib::AcariError;
use std::io::{stdout, Write};
use text_io::try_read;

pub fn init() -> Result<(), AcariError> {
  print!("Mite domain: ");
  stdout().flush()?;
  let domain: String = try_read!("{}\n").map_err(|e| AcariError::InternalError(format!("IO: {}", e)))?;
  print!("API Token: ");
  stdout().flush()?;
  let token: String = try_read!("{}\n").map_err(|e| AcariError::InternalError(format!("IO: {}", e)))?;

  Config {
    domain,
    token,
    cache_ttl_minutes: 1440,
  }
  .write()?;

  println!("Configuration updated");

  Ok(())
}
