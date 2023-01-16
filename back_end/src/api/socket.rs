use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::fs;
use actix::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::logger::{log_event, EventLog};

/// Define HTTP actor
struct MyApp {
    clients: HashMap<usize, Addr<Client>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrisonerCell {
    pub prisoner_id: i32,
    pub cell_id: i32,
}

struct Client {
    id: usize,
    addr: Addr<MyApp>,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self, MyApp>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr.do_send(Connect {
            id: self.id,
            addr: ctx.address(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // handle websocket messages here
    }
}

struct Connect {
    id: usize,
    addr: Addr<Client>,
}

impl Message for Connect {
    type Result = ();
}

struct Disconnect {
    id: usize,
}

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for MyApp {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.clients.remove(&msg.id);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyApp {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
                println!("{:?}", msg);
            }
            Ok(ws::Message::Pong(msg)) => {
                ctx.ping(&msg);
                println!("{:?}", msg);
            }
            Ok(ws::Message::Text(_)) => {
                let mut text = "test";

                ctx.binary("test");
            }

            /*
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
                    let msg = ws::Message::Text("lel");
                    ctx.address().do_send(msg)
                }
            
            
            },*/
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyApp {}, &req, stream);
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