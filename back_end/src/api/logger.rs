use actix_web::{get, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EventLog {
    pub initializer: String,
    pub event: String,
    pub date: DateTime<Utc>,
    pub ip: String,
}

impl EventLog {
    pub fn new(initializer: &str, event: &str, ip: &str) -> Self {
        Self {
            initializer: initializer.to_string(),
            event: event.to_string(),
            date: Utc::now(),
            ip: ip.to_string(),
        }
    }
}

pub fn log_event(event_log: EventLog) {

    let data = fs::read_to_string("./data/logs.json").expect("Unable to read file");

    let old_logs: Vec<EventLog> = serde_json::from_str(&data).unwrap();

    let mut logs: Vec<EventLog> = old_logs;

    logs.push(event_log);

    let text = serde_json::to_string(&logs).unwrap();

    fs::write("./data/logs.json",text).expect("Unable to write to file");
}

#[get("/logs")]
pub async fn get_logs() -> impl Responder {
    let data = fs::read_to_string("./data/logs.json").expect("Unable to read file");

    let logs: Vec<EventLog> = serde_json::from_str(&data).unwrap();

    HttpResponse::Ok().json(logs)
}