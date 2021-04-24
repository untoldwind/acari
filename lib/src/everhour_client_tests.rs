use super::{Account, AccountId, Client, Customer, CustomerId, EverhourClient, Project, ProjectId, Service, ServiceId, User, UserId};
use chrono::{TimeZone, Utc};
use pact_consumer::prelude::*;
use pact_consumer::term;
use serde_json::json;

const CONSUMER: &str = "acari-lib";
const PROVIDER: &str = "everhour API";

#[test]
fn test_get_account() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get account", |i| {
      i.given("User with API token");
      i.request.get().path("/users/me").header("X-Api-Key", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
            "id": 12345,
            "name": "August Ausgedacht",
            "email": "august.ausgedacht@demo.de",
            "status": "active",
            "role": "member",
            "headline": "",
            "isSuspended": false,
            "createdAt": "2021-01-29 12:00:50",
            "accounts": [{

            }],
            "team": {
                "id": 1234,
                "name": "Demo GmbH",
                "createdAt": "2021-01-14 18:59:59",
                "currencyDetails": {
                  "code": "EUR",
                  "name": "Euro",
                  "symbol": "€",
                  "favorite": 2
                }
            },
      }));
    })
    .build();
  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = EverhourClient::new_form_url(url);

  let account = client.get_account()?;

  assert_eq!(
    Account {
      id: AccountId::Num(1234),
      name: "Demo GmbH".to_string(),
      title: "Demo GmbH".to_string(),
      currency: "EUR".to_string(),
      created_at: Utc.ymd(2021, 1, 14).and_hms(18, 59, 59),
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
      i.request.get().path("/users/me").header("X-Api-Key", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
            "id": 12345,
            "name": "August Ausgedacht",
            "email": "august.ausgedacht@demo.de",
            "status": "active",
            "role": "member",
            "headline": "",
            "isSuspended": false,
            "createdAt": "2021-01-29 12:00:50",
            "accounts": [{

            }],
            "team": {
                "id": 1234,
                "name": "Demo GmbH",
                "createdAt": "2021-01-14 18:59:59",
                "currencyDetails": {
                  "code": "EUR",
                  "name": "Euro",
                  "symbol": "€",
                  "favorite": 2
                }
            },
      }));
    })
    .build();
  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = EverhourClient::new_form_url(url);

  let user = client.get_myself()?;

  assert_eq!(
    User {
      id: UserId::Num(12345),
      name: "August Ausgedacht".to_string(),
      email: "august.ausgedacht@demo.de".to_string(),
      note: "".to_string(),
      archived: false,
      role: "member".to_string(),
      language: "".to_string(),
      created_at: Utc.ymd(2021, 1, 29).and_hms(12, 0, 50),
    },
    user
  );

  Ok(())
}

#[test]
fn test_get_customers() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get projects", |i| {
      i.given("User with API token");
      i.request.get().path("/projects").header("X-Api-Key", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
        "id": "as:12345",
        "platform": "as",
        "name": "Project 1",
        "createdAt": "2021-01-14",
        "workspaceId": "as:54321",
        "workspaceName": "Workspace 1",
        "foreign": false,
        "status": "archived",
        "estimatesType": "any",
      }, {
        "id": "as:12346",
        "platform": "as",
        "name": "Project 2",
        "createdAt": "2021-01-15",
        "workspaceId": "as:54322",
        "workspaceName": "Workspace 2",
        "foreign": false,
        "status": "open",
        "estimatesType": "any",
      }, {
        "id": "as:12347",
        "platform": "as",
        "name": "Project 3",
        "createdAt": "2021-01-16",
        "workspaceId": "as:54321",
        "workspaceName": "Workspace 1",
        "foreign": false,
        "status": "open",
        "estimatesType": "any",
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = EverhourClient::new_form_url(url);

  let mut customers = client.get_customers()?;
  customers.sort_by(|c1, c2| c1.name.cmp(&c2.name));

  assert_eq!(customers.len(), 2);
  assert_eq!(
    Customer {
      id: CustomerId::Str("as:54321".to_string()),
      name: "Workspace 1".to_string(),
      note: "".to_string(),
      archived: false,
      created_at: Utc.ymd(2021, 01, 14).and_hms(00, 00, 00),
    },
    customers[0]
  );
  assert_eq!(
    Customer {
      id: CustomerId::Str("as:54322".to_string()),
      name: "Workspace 2".to_string(),
      note: "".to_string(),
      archived: false,
      created_at: Utc.ymd(2021, 01, 15).and_hms(00, 00, 00),
    },
    customers[1]
  );

  Ok(())
}

#[test]
fn test_get_projects() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get projects", |i| {
      i.given("User with API token");
      i.request.get().path("/projects").header("X-Api-Key", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
        "id": "as:12345",
        "platform": "as",
        "name": "Project 1",
        "createdAt": "2021-01-14",
        "workspaceId": "as:54321",
        "workspaceName": "Workspace 1",
        "foreign": false,
        "status": "archived",
        "estimatesType": "any",
      }, {
        "id": "as:12346",
        "platform": "as",
        "name": "Project 2",
        "createdAt": "2021-01-15",
        "workspaceId": "as:54322",
        "workspaceName": "Workspace 2",
        "foreign": false,
        "status": "open",
        "estimatesType": "any",
      }, {
        "id": "as:12347",
        "platform": "as",
        "name": "Project 3",
        "createdAt": "2021-01-16",
        "workspaceId": "as:54321",
        "workspaceName": "Workspace 1",
        "foreign": false,
        "status": "open",
        "estimatesType": "any",
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = EverhourClient::new_form_url(url);

  let projects = client.get_projects()?;

  assert_eq!(projects.len(), 3);
  assert_eq!(
    Project {
      id: ProjectId::Str("as:12345".to_string()),
      name: "Project 1".to_string(),
      note: "".to_string(),
      customer_id: CustomerId::Str("as:54321".to_string()),
      customer_name: "Workspace 1".to_string(),
      archived: true,
      created_at: Utc.ymd(2021, 01, 14).and_hms(00, 00, 00),
    },
    projects[0]
  );
  assert_eq!(
    Project {
      id: ProjectId::Str("as:12346".to_string()),
      name: "Project 2".to_string(),
      note: "".to_string(),
      customer_id: CustomerId::Str("as:54322".to_string()),
      customer_name: "Workspace 2".to_string(),
      archived: false,
      created_at: Utc.ymd(2021, 01, 15).and_hms(00, 00, 00),
    },
    projects[1]
  );
  assert_eq!(
    Project {
      id: ProjectId::Str("as:12347".to_string()),
      name: "Project 3".to_string(),
      note: "".to_string(),
      customer_id: CustomerId::Str("as:54321".to_string()),
      customer_name: "Workspace 1".to_string(),
      archived: false,
      created_at: Utc.ymd(2021, 01, 16).and_hms(00, 00, 00),
    },
    projects[2]
  );

  Ok(())
}

#[test]
fn test_get_services() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get project tasks", |i| {
      i.given("User with API token");
      i.request
        .get()
        .path("/projects/as%3A12345/tasks")
        .header("X-Api-Key", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([{
        "id": "as:123451234",
        "name": "Task 1",
        "iteration": "Untitled section",
        "createdAt": "2021-01-18 12:55:55",
        "status": "closed",
        "projects": [
          "as:8353429"
        ],
      }, {
        "id": "as:123451235",
        "name": "Task 2",
        "iteration": "Untitled section",
        "createdAt": "2021-01-25 11:54:28",
        "status": "open",
        "projects": [
          "as:8353429"
        ],
      }]));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = EverhourClient::new_form_url(url);

  let services = client.get_services(&ProjectId::Str("as:12345".to_string()))?;

  assert_eq!(services.len(), 2);
  assert_eq!(
    Service {
      id: ServiceId::Str("as:123451234".to_string()),
      name: "Task 1".to_string(),
      note: "Untitled section".to_string(),
      archived: true,
      billable: true,
      created_at: Utc.ymd(2021, 01, 18).and_hms(12, 55, 55),
    },
    services[0]
  );
  assert_eq!(
    Service {
      id: ServiceId::Str("as:123451235".to_string()),
      name: "Task 2".to_string(),
      note: "Untitled section".to_string(),
      archived: false,
      billable: true,
      created_at: Utc.ymd(2021, 01, 25).and_hms(11, 54, 28),
    },
    services[1]
  );

  Ok(())
}
