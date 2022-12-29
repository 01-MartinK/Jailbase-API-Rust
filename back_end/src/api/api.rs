use actix_web::{get, post, delete, patch, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Criminal {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub dob: String,
    pub name: String,
    pub crime: String,
    pub extras: String,
    pub image_link: String,
}

#[get("/criminals")]
pub async fn get_all_criminals() -> impl Responder {
    
    let criminals: Vec<Criminal> = get_from_json();

    HttpResponse::Ok().json(criminals)
}

#[post("/criminals/add")]
pub async fn add_criminal(req_body: String) -> impl Responder {
    let crim: Criminal = serde_json::from_str(&req_body).unwrap();

    //println!("{}", req_body);
    //println!("{:?}", crim);

    let mut criminals: Vec<Criminal> = get_from_json();
    criminals.push(crim);

    write_to_json(criminals);

    HttpResponse::Ok()
}

#[patch("/criminals/update")]
pub async fn update_criminal(req_body: String) -> impl Responder {
    let updated_crim: Criminal = serde_json::from_str(&req_body).unwrap();

    let needed_id = updated_crim.clone().id;

    let criminals: Vec<Criminal> = get_from_json();
    let mut new_criminals: Vec<Criminal> = Vec::new();

    for i in criminals {
        if i.id == needed_id {
            new_criminals.push(updated_crim.clone());
        }
        else {
            new_criminals.push(i);
        }
    }

    write_to_json(new_criminals);

    HttpResponse::Ok()
}

#[delete("/criminals/delete")]
pub async fn delete_criminal(req_body: String) -> impl Responder {
    let index: usize = req_body.parse::<usize>().unwrap();

    let mut criminals: Vec<Criminal> = get_from_json();

    criminals.remove(index);

    write_to_json(criminals);
    
    //println!("{}", index);

    HttpResponse::Ok()
}

fn write_to_json(crim_list : Vec<Criminal>) {
    let text = serde_json::to_string(&crim_list).unwrap();

    println!("{}", text);
    fs::write("./data/data.json",text).expect("Unable to write to file");
}

fn get_from_json() -> Vec<Criminal> {
    let data = fs::read_to_string("./data/data.json").expect("Unable to read file");

    let criminals: Vec<Criminal> = serde_json::from_str(&data).unwrap();
    //println!("{:?}", criminals);

    return criminals;
}