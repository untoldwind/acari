use chrono::{NaiveDate, TimeZone, Utc};
use pact_consumer::prelude::*;
use pact_consumer::term;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::{
  Account, AccountId, Client, Customer, CustomerId, DateSpan, Day, Minutes, Project, ProjectId, Service, ServiceId, StdClient, TimeEntry, TimeEntryId, Tracker,
  TrackingTimeEntry, User, UserId,
};

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

#[test]
fn test_query_entries() -> Result<(), Box<dyn std::error::Error>> {
  let time_entry_json = json!({
    "time_entry": {
       "id": 36159117,
       "minutes": 15,
       "date_at": "2015-10-16",
       "note": "Feedback einarbeiten",
       "billable": true,
       "locked": false,
       "revenue": null,
       "hourly_rate": 0,
       "user_id": 211,
       "user_name": "Fridolin Frei",
       "project_id": 88309,
       "project_name": "API v2",
       "customer_id": 3213,
       "customer_name": "König",
       "service_id": 12984,
       "service_name": "Entwurf",
       "created_at": "2015-10-16T12:19:00+02:00",
       "updated_at": "2015-10-16T12:39:00+02:00"
    }
  });
  let expected = TimeEntry {
    id: TimeEntryId(36159117),
    minutes: Minutes(15),
    date_at: NaiveDate::from_ymd(2015, 10, 16),
    note: "Feedback einarbeiten".to_string(),
    locked: false,
    billable: true,
    hourly_rate: 0,
    user_id: UserId(211),
    user_name: "Fridolin Frei".to_string(),
    customer_id: CustomerId(3213),
    customer_name: "König".to_string(),
    service_id: ServiceId(12984),
    service_name: "Entwurf".to_string(),
    project_id: ProjectId(88309),
    project_name: "API v2".to_string(),
    created_at: Utc.ymd(2015, 10, 16).and_hms(10, 19, 00),
    updated_at: Utc.ymd(2015, 10, 16).and_hms(10, 39, 00),
  };

  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("query time entries", |i| {
      i.given("User with API token");
      i.request
        .get()
        .path("/time_entries.json")
        .query_param("at", "2015-10-16")
        .query_param("user", "current")
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!([time_entry_json]));
    })
    .interaction("get time entry by id", |i| {
      i.given("User with API token");
      i.request
        .get()
        .path("/time_entries/36159117.json")
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(time_entry_json);
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let entries = client.get_time_entries(DateSpan::Day(Day::Date(NaiveDate::from_ymd(2015, 10, 16))))?;

  assert_eq!(entries.len(), 1);
  assert_eq!(expected, entries[0]);

  let entry = client.get_time_entry(TimeEntryId(36159117))?;

  assert_eq!(expected, entry);

  Ok(())
}

#[test]
fn test_create_entry() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("create time entry", |i| {
      i.given("User with API token");
      i.request
        .post()
        .path("/time_entries.json")
        .json_body(json!({
           "time_entry": {
              "date_at": "2015-09-15",
              "minutes": 185,
              "project_id": 3456,
              "service_id": 243,
              "note": "",
           }
        }))
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.created().json_utf8().json_body(json!({
         "time_entry": {
            "id": 52324,
            "minutes": 185,
            "date_at": "2015-9-12",
            "note": "",
            "billable": true,
            "locked": false,
            "revenue": null,
            "hourly_rate": 0,
            "user_id": 211,
            "user_name": "Fridolin Frei",
            "customer_id": 3213,
            "customer_name": "König",
            "project_id": 3456,
            "project_name": "Some project",
            "service_id": 243,
            "service_name": "Dokumentation",
            "created_at": "2015-09-13T18:54:45+02:00",
            "updated_at": "2015-09-13T18:54:45+02:00"
         }
      }));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let entry = client.create_time_entry(Day::Date(NaiveDate::from_ymd(2015, 9, 15)), ProjectId(3456), ServiceId(243), Minutes(185), None)?;

  assert_eq!(
    TimeEntry {
      id: TimeEntryId(52324),
      minutes: Minutes(185),
      date_at: NaiveDate::from_ymd(2015, 9, 12),
      note: "".to_string(),
      locked: false,
      billable: true,
      hourly_rate: 0,
      user_id: UserId(211),
      user_name: "Fridolin Frei".to_string(),
      customer_id: CustomerId(3213),
      customer_name: "König".to_string(),
      service_id: ServiceId(243),
      service_name: "Dokumentation".to_string(),
      project_id: ProjectId(3456),
      project_name: "Some project".to_string(),
      created_at: Utc.ymd(2015, 9, 13).and_hms(16, 54, 45),
      updated_at: Utc.ymd(2015, 9, 13).and_hms(16, 54, 45),
    },
    entry
  );

  Ok(())
}

#[test]
fn test_delete_entry() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("delete time entry", |i| {
      i.given("User with API token");
      i.request
        .delete()
        .path("/time_entries/52324.json")
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok();
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  client.delete_time_entry(TimeEntryId(52324))?;

  Ok(())
}

#[test]
fn test_update_entry() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("update time entry", |i| {
      i.given("User with API token");
      i.request
        .method("PATCH")
        .path("/time_entries/52324.json")
        .json_body(json!({
           "time_entry": {
              "minutes": 120,
              "note": "",
           }
        }))
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok();
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  client.update_time_entry(TimeEntryId(52324), Minutes(120), None)?;

  Ok(())
}

#[test]
fn test_get_tracker() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("get tracker", |i| {
      i.given("User with API token");
      i.request.get().path("/tracker.json").header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
        "tracker": {
          "tracking_time_entry": {
            "id": 36135321,
            "minutes": 247,
            "since": "2015-10-15T17:05:04+02:00"
          }
        }
      }));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let tracker = client.get_tracker()?;

  assert_eq!(
    Tracker {
      tracking_time_entry: Some(TrackingTimeEntry {
        id: TimeEntryId(36135321),
        minutes: Minutes(247),
        since: Some(Utc.ymd(2015, 10, 15).and_hms(15, 05, 04))
      }),
      stopped_time_entry: None,
    },
    tracker
  );

  Ok(())
}

#[test]
fn test_create_tracker() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("create tracker", |i| {
      i.given("User with API token");
      i.request
        .method("PATCH")
        .path("/tracker/36135322.json")
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
        "tracker": {
          "tracking_time_entry": {
            "id": 36135322,
            "minutes": 0,
            "since": "2015-10-15T17:33:52+02:00"
          },
         "stopped_time_entry": {
            "id": 36134329,
            "minutes": 46
          }
        }
      }));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let tracker = client.create_tracker(TimeEntryId(36135322))?;

  assert_eq!(
    Tracker {
      tracking_time_entry: Some(TrackingTimeEntry {
        id: TimeEntryId(36135322),
        minutes: Minutes(0),
        since: Some(Utc.ymd(2015, 10, 15).and_hms(15, 33, 52)),
      }),
      stopped_time_entry: Some(TrackingTimeEntry {
        id: TimeEntryId(36134329),
        minutes: Minutes(46),
        since: None,
      }),
    },
    tracker
  );

  Ok(())
}

#[test]
fn test_delete_tracker() -> Result<(), Box<dyn std::error::Error>> {
  let pact = PactBuilder::new(CONSUMER, PROVIDER)
    .interaction("delete tracker", |i| {
      i.given("User with API token");
      i.request
        .delete()
        .path("/tracker/36135322.json")
        .header("X-MiteApiKey", term!("[0-9a-f]+", "12345678"));
      i.response.ok().json_utf8().json_body(json!({
        "tracker": {
         "stopped_time_entry": {
            "id": 36135322,
            "minutes": 4
          }
        }
      }));
    })
    .build();

  let server = pact.start_mock_server();
  let mut url = server.url().clone();
  url.set_username("12345678").unwrap();
  let client = StdClient::new_form_url(url);

  let tracker = client.delete_tracker(TimeEntryId(36135322))?;

  assert_eq!(
    Tracker {
      tracking_time_entry: None,
      stopped_time_entry: Some(TrackingTimeEntry {
        id: TimeEntryId(36135322),
        minutes: Minutes(4),
        since: None,
      }),
    },
    tracker
  );

  Ok(())
}
