use crate::config::Config;
use std::io::{stdout, Write};
use text_io::try_read;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
  print!("Mite domain: ");
  stdout().flush()?;
  let domain: String = try_read!("{}\n")?;
  print!("API Token: ");
  stdout().flush()?;
  let token: String = try_read!("{}\n")?;

  Config {
    domain,
    token,
    cache_ttl_minutes: 1440,
  }
  .write()?;

  println!("Configuration updated");

  Ok(())
}
