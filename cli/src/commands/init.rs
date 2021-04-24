use crate::config::{ClientType, Config, Profile};
use std::io::{stdout, Write};
use text_io::try_read;

pub fn init(maybe_existing_config: Option<Config>, maybe_profile: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
  let mut config = match maybe_existing_config {
    Some(existing) => {
      let confirm = match maybe_profile {
        Some(profile) if existing.profiles.contains_key(profile) => Some(format!("Overwrite existing {} profile (yes/No): ", profile)),
        None if !existing.domain.is_empty() || !existing.token.is_empty() => Some("Overwrite existing default profile (yes/No): ".to_string()),
        _ => None,
      };
      if let Some(msg) = confirm {
        if console_input(&msg)? != "yes" {
          return Ok(());
        }
      }
      existing
    }
    None => Config {
      cache_ttl_minutes: 1440,
      ..Default::default()
    },
  };

  let domain: String = console_input("Mite domain: ")?;
  let token: String = console_input("API Token: ")?;

  match maybe_profile {
    Some(profile) => {
      config.profiles.insert(
        profile.to_string(),
        Profile {
          domain,
          token,
          client: ClientType::Mite,
        },
      );
    }
    None => {
      config.domain = domain;
      config.token = token;
    }
  };
  config.write()?;

  println!("Configuration updated");

  Ok(())
}

fn console_input(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
  print!("{}", msg);
  stdout().flush()?;

  let input: String = try_read!("{}\n")?;

  Ok(input.trim().to_string())
}
