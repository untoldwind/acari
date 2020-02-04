use chrono::{TimeZone, Utc};
use pact_consumer::prelude::*;
use pact_consumer::term;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::{Account, AccountId, Client, StdClient, User, UserId};

const CONSUMER: &str = "acari-lib";
const PROVIDER: &str = "mite API";

#[test]
fn test_get_account() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get account", |i| {
      i.given("User with API token");
      i.request.get().path("/account.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
          "account": {
              "id": 1,
              "name": "demo",
              "title": "Demo GmbH",
              "currency": "EUR",
              "created_at": "2013-10-12T14:39:51+01:00",
              "updated_at": "2015-05-02T13:21:09+01:00"
          }
      }));
    })
    .build();
  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let account = client.get_account()?;

  assert_eq!(
    Account {
      id: AccountId(1),
      name: "demo".to_string(),
      title: "Demo GmbH".to_string(),
      currency: "EUR".to_string(),
      created_at: Utc.ymd(2013, 10, 12).and_hms(13, 39, 51),
      updated_at: Utc.ymd(2015, 5, 2).and_hms(12, 21, 09),
    },
    account
  );

  Ok(())
}

#[test]
fn test_get_myself() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get myself", |i| {
      i.given("User with API token");
      i.request.get().path("/myself.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
          "user": {
              "id": 3456,
              "name": "August Ausgedacht",
              "email": "august.ausgedacht@demo.de",
              "note": "",
              "archived": false,
              "role": "admin",
              "language": "de",
              "created_at": "2013-06-23T23:00:58+02:00",
              "updated_at": "2015-07-25T01:26:35+02:00"
          }
      }));
    })
    .build();
  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let user = client.get_myself()?;

  assert_eq!(
    User {
      id: UserId(3456),
      name: "August Ausgedacht".to_string(),
      email: "august.ausgedacht@demo.de".to_string(),
      note: "".to_string(),
      archived: false,
      role: "admin".to_string(),
      language: "de".to_string(),
      created_at: Utc.ymd(2013, 6, 23).and_hms(21, 0, 58),
      updated_at: Utc.ymd(2015, 7, 24).and_hms(23, 26, 35),
    },
    user
  );

  Ok(())
}
