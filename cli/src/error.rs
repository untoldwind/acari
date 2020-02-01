use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
  IO(io::Error),
  TextIO(text_io::Error),
  TomlRead(toml::de::Error),
  TomlWrite(toml::ser::Error),
  Json(serde_json::Error),
  AcariError(acari_lib::AcariError),
  UserError(String),
  InternalError(String),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppError::IO(err) => write!(f, "IO error: {}", err),
      AppError::TextIO(err) => write!(f, "IO error: {}", err),
      AppError::TomlRead(err) => write!(f, "Toml error: {}", err),
      AppError::TomlWrite(err) => write!(f, "Toml error: {}", err),
      AppError::Json(err) => write!(f, "Json error: {}", err),
      AppError::AcariError(err) => write!(f, "Error: {}", err),
      AppError::UserError(s) => write!(f, "User error: {}", s),
      AppError::InternalError(s) => write!(f, "Internal error: {}", s),
    }
  }
}

impl Error for AppError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      AppError::IO(err) => Some(err),
      AppError::TextIO(err) => Some(err),
      AppError::TomlRead(err) => Some(err),
      AppError::TomlWrite(err) => Some(err),
      AppError::Json(err) => Some(err),
      AppError::AcariError(err) => Some(err),
      _ => None,
    }
  }
}

macro_rules! app_error_from {
  ($error: ty, $app_error: ident) => {
    impl From<$error> for AppError {
      fn from(err: $error) -> AppError {
        AppError::$app_error(err)
      }
    }
  };
}

app_error_from!(io::Error, IO);
app_error_from!(text_io::Error, TextIO);
app_error_from!(toml::de::Error, TomlRead);
app_error_from!(toml::ser::Error, TomlWrite);
app_error_from!(serde_json::Error, Json);
app_error_from!(acari_lib::AcariError, AcariError);
