use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::api::logger::{log_event, EventLog};

/// Define HTTP actor
struct MyWs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrisonerCell {
    pub prisoner_id: i32,
    pub cell_id: i32,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {

                if &text != "" {
                    let prisoner_cell: PrisonerCell = serde_json::from_str(&text).unwrap();
                    
                    let prisoner_cells: Vec<PrisonerCell> = get_from_json().clone();
                    let mut new_cells: Vec<PrisonerCell> = Vec::new();

                    let mut iter = prisoner_cells.iter();
                    let cell = iter.find(|cell| cell.prisoner_id == prisoner_cell.prisoner_id);

                    if !cell.is_none() {
                        for c in prisoner_cells {
                            if c.prisoner_id == prisoner_cell.prisoner_id {
                                new_cells.push(prisoner_cell.clone());
                            }else {
                                new_cells.push(c);
                            }
                        }
                    }else {
                        new_cells.clone_from(&prisoner_cells);
                        new_cells.push(prisoner_cell);
                    }

                    log_event(EventLog::new("Admin", "Change of cells", "192.168.47.5"));

                    write_to_json(new_cells.clone());

                    let message = serde_json::to_string(&new_cells).unwrap();

                    ctx.text(message);
                }else {
                    let prisoner_cells: Vec<PrisonerCell> = get_from_json().clone();

                    ctx.text(serde_json::to_string(&prisoner_cells).unwrap());
                }
            
            
            },
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

fn write_to_json(cell_list : Vec<PrisonerCell>) {
    let text = serde_json::to_string(&cell_list).unwrap();

    println!("{}", text);
    fs::write("./data/cellData.json",text).expect("Unable to write to file");
}

fn get_from_json() -> Vec<PrisonerCell> {
    let data = fs::read_to_string("./data/cellData.json").expect("Unable to read file");

    let cells: Vec<PrisonerCell> = serde_json::from_str(&data).unwrap();
    //println!("{:?}", criminals);

    return cells;
}