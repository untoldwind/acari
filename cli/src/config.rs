use crate::error::AppError;
use acari_lib::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub domain: String,
  pub token: String,
}

impl Config {
  pub fn read() -> Result<Option<Config>, AppError> {
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

  pub fn client(&self) -> Client {
    Client::new(&self.domain, &self.token)
  }

  pub fn write(&self) -> Result<(), AppError> {
    let content = toml::to_string_pretty(self)?;
    let config_file = config_file();

    fs::create_dir_all(&config_file.parent().ok_or_else(|| AppError::InternalError("Invalid config path".to_string()))?)?;

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
