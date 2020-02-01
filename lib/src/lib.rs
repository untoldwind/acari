mod error;
mod model;

pub use error::AcariError;
pub use model::{Account, User};
pub use serde::de::DeserializeOwned;

use model::MiteResponse;

use reqwest::{blocking, header, StatusCode};

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

#[derive(Debug)]
pub struct Client {
  domain: String,
  token: String,
  client: blocking::Client,
}

impl Client {
  pub fn new(domain: &str, token: &str) -> Client {
    Client {
      domain: domain.to_string(),
      token: token.to_string(),
      client: blocking::Client::new(),
    }
  }

  fn get(&self, uri: &str) -> Result<MiteResponse, AcariError> {
    let response = self
      .client
      .get(&format!("https://{}{}", self.domain, uri))
      .header(header::USER_AGENT, USER_AGENT)
      .header(header::HOST, &self.domain)
      .header("X-MiteApiKey", &self.token)
      .send()?;

    handle_response(response)
  }

  pub fn get_account(&self) -> Result<Account, AcariError> {
    match self.get("/account.json")? {
      MiteResponse::Account(account) => Ok(account),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }

  pub fn get_myself(&self) -> Result<User, AcariError> {
    match self.get("/myself.json")? {
      MiteResponse::User(user) => Ok(user),
      response => Err(AcariError::Mite(400, format!("Unexpected response: {:?}", response))),
    }
  }
}

fn handle_response(response: blocking::Response) -> Result<MiteResponse, AcariError> {
  match response.status() {
    StatusCode::OK => Ok(response.json()?),
    status => match response.json::<MiteResponse>() {
      Ok(MiteResponse::Error(msg)) => Err(AcariError::Mite(status.as_u16(), msg)),
      _ => Err(AcariError::Mite(status.as_u16(), status.to_string())),
    },
  }
}
