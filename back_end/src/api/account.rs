use actix_web::{post, HttpResponse, Responder, web::Json};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::api::logger::{log_event, EventLog};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(req_body: Json<LoginRequest>) -> impl Responder {

    println!("{:?}", req_body);

    let users: Vec<User> = get_from_json();
    let mut iter = users.iter();
    let user = iter.find(|user| user.name == req_body.name && user.password == req_body.password);

    log_event(EventLog::new("no account", "Login to account Admin", "192.168.47.5"));

    println!("{:?}", user);

    HttpResponse::Ok().json(user)
}

fn get_from_json() -> Vec<User> {
    let data = fs::read_to_string("./data/accounts.json").expect("Unable to read file");

    let users: Vec<User> = serde_json::from_str(&data).unwrap();
    //println!("{:?}", Users);

    return users;
}