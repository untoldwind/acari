use chrono::{TimeZone, Utc};
use pact_consumer::prelude::*;
use pact_consumer::term;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::{Account, AccountId, Client, Customer, CustomerId, Project, ProjectId, Service, ServiceId, StdClient, User, UserId};

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

#[test]
fn test_get_customers() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get customers", |i| {
      i.given("User with API token");
      i.request.get().path("/customers.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
         "customer": {
            "id": 83241,
            "name": "Acme Inc.",
            "note": "",
            "archived": false,
            "active_hourly_rate": "hourly_rates_per_service",
            "hourly_rate": null,
            "hourly_rates_per_service": [
               {
                  "service_id": 742,
                  "hourly_rate": 4500
               },
               {
                  "service_id": 43212,
                  "hourly_rate": 5500
               }
            ],
            "created_at": "2015-10-15T14:33:19+02:00",
            "updated_at": "2015-10-15T14:29:03+02:00"
         }
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let customers = client.get_customers()?;

  assert_eq!(customers.len(), 1);
  assert_eq!(
    Customer {
      id: CustomerId(83241),
      name: "Acme Inc.".to_string(),
      note: "".to_string(),
      archived: false,
      hourly_rate: None,
      created_at: Utc.ymd(2015, 10, 15).and_hms(12, 33, 19),
      updated_at: Utc.ymd(2015, 10, 15).and_hms(12, 29, 03)
    },
    customers[0]
  );

  Ok(())
}

#[test]
fn test_get_projects() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get projects", |i| {
      i.given("User with API token");
      i.request.get().path("/projects.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
         "project": {
            "id": 643,
            "name": "Open-Source",
            "note": "valvat, memento et all.",
            "customer_id": 291,
            "customer_name": "Yolk",
            "budget": 0,
            "budget_type": "minutes",
            "hourly_rate": 6000,
            "archived": false,
            "active_hourly_rate": "hourly_rate",
            "hourly_rates_per_service": [
               {
                  "service_id": 31272,
                  "hourly_rate": 4500
               },
               {
                  "service_id": 149228,
                  "hourly_rate": 5500
               }
            ],
            "created_at": "2011-08-17T12:06:57+02:00",
            "updated_at": "2015-02-19T10:53:10+01:00"
         }
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let projects = client.get_projects()?;

  assert_eq!(projects.len(), 1);
  assert_eq!(
    Project {
      id: ProjectId(643),
      name: "Open-Source".to_string(),
      note: "valvat, memento et all.".to_string(),
      customer_id: CustomerId(291),
      customer_name: "Yolk".to_string(),
      budget: 0,
      budget_type: "minutes".to_string(),
      hourly_rate: Some(6000),
      archived: false,
      created_at: Utc.ymd(2011, 08, 17).and_hms(10, 06, 57),
      updated_at: Utc.ymd(2015, 02, 19).and_hms(09, 53, 10),
    },
    projects[0]
  );

  Ok(())
}

#[test]
fn test_get_services() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get services", |i| {
      i.given("User with API token");
      i.request.get().path("/services.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
         "service": {
              "id": 38672,
              "name": "Website Konzeption",
              "note": "",
              "hourly_rate": 3300,
              "archived": false,
              "billable": true,
              "created_at": "2009-12-13T12:12:00+01:00",
              "updated_at": "2015-12-13T07:20:04+01:00"
          }
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let services = client.get_services()?;

  assert_eq!(services.len(), 1);
  assert_eq!(
    Service {
      id: ServiceId(38672),
      name: "Website Konzeption".to_string(),
      note: "".to_string(),
      archived: false,
      billable: true,
      hourly_rate: Some(3300),
      created_at: Utc.ymd(2009, 12, 13).and_hms(11, 12, 00),
      updated_at: Utc.ymd(2015, 12, 13).and_hms(06, 20, 04)
    },
    services[0]
  );

  Ok(())
}
