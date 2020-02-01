use crate::config::Config;
use crate::error::AppError;
use std::io::{stdout, Write};
use text_io::try_read;

pub fn init() -> Result<(), AppError> {
  print!("Mite domain: ");
  stdout().flush()?;
  let domain: String = try_read!("{}\n")?;
  print!("API Token: ");
  stdout().flush()?;
  let token: String = try_read!("{}\n")?;

  Config { domain, token }.write()?;

  println!("Configuration updated");

  Ok(())
}
