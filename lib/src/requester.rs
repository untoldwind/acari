use std::marker::PhantomData;

use reqwest::{blocking, header, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::AcariError;

const USER_AGENT: &str = "acari-lib (https://github.com/untoldwind/acari)";

pub trait Requester {
  fn request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError>;

  fn caching_request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError>;

  fn request_empty(&self, method: Method, uri: &str) -> Result<(), AcariError>;

  fn request_with_body<T: DeserializeOwned, D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<T, AcariError>;

  fn request_empty_with_body<D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<(), AcariError>;
}

pub trait ResponseHandler {
  fn handle_response<T: DeserializeOwned>(response: blocking::Response) -> Result<T, AcariError>;

  fn handle_empty_response(response: blocking::Response) -> Result<(), AcariError>;
}

pub struct BaseRequester<H: ResponseHandler> {
  base_url: Url,
  client: blocking::Client,
  extra_headers: header::HeaderMap,
  response_handler: PhantomData<H>,
}

impl<H: ResponseHandler> BaseRequester<H> {
  fn base_request(&self, method: Method, uri: &str) -> Result<blocking::RequestBuilder, AcariError> {
    Ok(
      self
        .client
        .request(method, self.base_url.join(uri)?.as_str())
        .header(header::USER_AGENT, USER_AGENT)
        .header(header::HOST, self.base_url.host_str().unwrap_or(""))
        .headers(self.extra_headers.clone()),
    )
  }
}

impl<H: ResponseHandler> Requester for BaseRequester<H> {
  fn request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    H::handle_response(response)
  }

  fn caching_request<T: DeserializeOwned>(&self, method: Method, uri: &str) -> Result<T, AcariError> {
    self.request(method, uri)
  }

  fn request_empty(&self, method: Method, uri: &str) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.send()?;

    H::handle_empty_response(response)
  }

  fn request_with_body<T: DeserializeOwned, D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<T, AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    H::handle_response(response)
  }

  fn request_empty_with_body<D: Serialize>(&self, method: Method, uri: &str, data: D) -> Result<(), AcariError> {
    let response = self.base_request(method, uri)?.json(&data).send()?;

    H::handle_empty_response(response)
  }
}
