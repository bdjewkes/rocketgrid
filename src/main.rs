#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(type_ascription)]

mod cors;
extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
extern crate ws;

use cors::CORS;
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket_contrib::{JSON, Value};
use std::collections::HashMap;
use std::sync::{RwLock,Mutex};
use std::thread;
use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};

#[derive(Serialize, Deserialize)]
struct Point(usize, usize);
impl Point {
    fn get_x(&self) -> usize {
        self.0
    }
    fn get_y(&self) -> usize {
        self.1
    }

    fn up(&mut self, distance: usize) {
        if self.get_y() < HEIGHT - distance {
            self.1 += distance;
        }
    }
    fn down(&mut self, distance: usize) {
        if self.get_y() >= distance {
            self.1 -= distance;
        }
    }
    fn left(&mut self, distance: usize) {
        if self.get_x() >= distance {
            self.0 -= distance;
        }
    }
    fn right(&mut self, distance: usize) {
        if self.get_x() < WIDTH - distance {
            self.0 += distance;
        }
    }
    fn json(&self) -> JSON<Value> {
        JSON(json!(&self))
    }
}

type PositionMap = RwLock<HashMap<usize, Point>>;

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[get("/position/reset")]
fn reset(position_map_state: State<PositionMap>) -> CORS<JSON<Value>> {
    let mut position_map = position_map_state.write().unwrap();
    position_map.clear();
    CORS::any(JSON(json!("OK")))
}

#[get("/position")]
fn position(position_map_state: State<PositionMap>) -> CORS<JSON<Value>> {
    let position_map = position_map_state.read().unwrap();
    let results: Vec<&Point> = position_map.iter().map(|(_, point)| point).collect();
    send_message(serde_json::to_string(&results).unwrap());
    CORS::any(JSON(json!(results)))
}

#[get("/position/new")]
fn new_position(entity_count: State<AtomicUsize>,
                position_map_state: State<PositionMap>)
                -> CORS<JSON<Value>> {
    let mut position_map = position_map_state.write().unwrap();
    let position_num = entity_count.fetch_add(1, Ordering::Relaxed);
    let point = Point(0, 0);
    position_map.entry(position_num).or_insert(point);
    CORS::any(JSON(json!(position_num)))
}

#[get("/position/<id>")]
fn get_entities(id: usize, position_map_state: State<PositionMap>) -> CORS<Option<JSON<Value>>> {
    let position_map = position_map_state.read().unwrap();
    match position_map.get(&id) {
        Some(position) => CORS::any(Some(position.json())),
        None => CORS::any(None),
    }
}

#[get("/position/<id>/<direction>")]
fn position_move(id: usize,
                 direction: &str,
                 position_map_state: State<PositionMap>)
                 -> CORS<Option<JSON<Value>>> {
    let mut position_map = position_map_state.write().unwrap();

    let result = match position_map.get_mut(&id) {
        Some(position) => {
            match direction {
                "left" => position.left(1),
                "right" => position.right(1),
                "down" => position.down(1),
                "up" => position.up(1),
                _ => return CORS::any(None),
            }
            CORS::any(Some(position.json()))
        }
        None => CORS::any(None),
    };   
    let state: Vec<&Point> = position_map.iter().map(|(_, point)| point).collect();
    send_message(serde_json::to_string(&state).unwrap());
    result
}

struct Server {
        out: Sender,
}
impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        self.out.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

fn send_message(message: String) {
    let client = thread::spawn(move || {
        let m = message.as_str();
        connect("ws://127.0.0.1:3012", |out| {
            out.send(m);

            move |msg| {
                println!("{}", msg);
                out.close(CloseCode::Normal)
            }
        })
    });
}

fn main() {
    // Server thread
    let server = thread::spawn(move || {
        listen("127.0.0.1:3012", |out| {
            Server { out: out }
        })
    });
    
    rocket::ignite()
        .mount("/",
               routes![position, position_move, get_entities, new_position, reset])
        .manage(AtomicUsize::new(0))
        .manage(RwLock::new(HashMap::<usize, Point>::new()))
        .manage(Mutex::new(server))
        .launch();
}
