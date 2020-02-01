use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AcariError {
  IO(io::Error),
  DateFormat(chrono::format::ParseError),
  Request(reqwest::Error),
  Mite(u16, String),
}

impl fmt::Display for AcariError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AcariError::IO(err) => write!(f, "IO error: {}", err),
      AcariError::DateFormat(err) => write!(f, "Date format error: {}", err),
      AcariError::Request(err) => write!(f, "Request error: {}", err),
      AcariError::Mite(status, error) => write!(f, "Mite error ({}): {}", status, error),
    }
  }
}

impl Error for AcariError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      AcariError::IO(err) => Some(err),
      AcariError::DateFormat(err) => Some(err),
      AcariError::Request(err) => Some(err),
      _ => None,
    }
  }
}

macro_rules! acari_error_from {
  ($error: ty, $app_error: ident) => {
    impl From<$error> for AcariError {
      fn from(err: $error) -> AcariError {
        AcariError::$app_error(err)
      }
    }
  };
}

acari_error_from!(io::Error, IO);
acari_error_from!(chrono::format::ParseError, DateFormat);
acari_error_from!(reqwest::Error, Request);
