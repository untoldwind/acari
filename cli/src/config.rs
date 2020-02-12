use acari_lib::{internal_error, AcariError, CachedClient, Client, StdClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
  pub domain: String,
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
  pub domain: String,
  pub token: String,
  #[serde(default = "default_cache_ttl")]
  pub cache_ttl_minutes: u64,
  #[serde(default)]
  pub profiles: HashMap<String, Profile>,
}

impl Config {
  pub fn read() -> Result<Option<Config>, Box<dyn std::error::Error>> {
    let config_file = config_file();
    match File::open(&config_file) {
      Ok(mut file) => {
        let mut content = vec![];
        file.read_to_end(&mut content)?;
        Ok(Some(toml::from_slice::<Config>(&content)?))
      }
      Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
      Err(err) => Err(err.into()),
    }
  }

  pub fn client(&self, maybe_profile: Option<&str>, cached: bool) -> Result<Box<dyn Client>, AcariError> {
    let (domain, token) = match maybe_profile {
      Some(profile_name) => {
        let profile = self
          .profiles
          .get(profile_name)
          .ok_or_else(|| AcariError::UserError(format!("No such profile: {}", profile_name)))?;
        (&profile.domain, &profile.token)
      }
      None => (&self.domain, &self.token),
    };
    if cached {
      Ok(Box::new(CachedClient::new(domain, token, Duration::from_secs(self.cache_ttl_minutes * 60))?))
    } else {
      Ok(Box::new(StdClient::new(domain, token)?))
    }
  }

  pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string_pretty(self)?;
    let config_file = config_file();

    fs::create_dir_all(
      &config_file
        .parent()
        .ok_or_else(|| internal_error!("Invalid config path: {}", config_file.to_string_lossy()))?,
    )?;

    let mut file = File::create(&config_file)?;

    file.write_all(content.as_bytes())?;

    Ok(())
  }
}

fn config_file() -> PathBuf {
  let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
  dirs::config_dir()
    .map(|configs| configs.join("acari"))
    .unwrap_or_else(|| home_dir.join(".acari"))
    .join("config.toml")
}

fn default_cache_ttl() -> u64 {
  1440
}
