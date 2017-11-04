#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(type_ascription)]

extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
extern crate ws;

mod cors;
mod grid;
mod statesocket;
mod entity;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::{RwLock,Mutex};
use std::thread;
use rocket::State;
use rocket::http::{RawStr};
use rocket_contrib::{Json, Value};
use ws::{listen};
use statesocket::{Server, send_message};
use entity::Entity;
use grid::Point;
use cors::CORS;

type EntityMap = RwLock<HashMap<usize, Entity>>;

#[get("/position/reset")]
fn reset(position_map_state: State<EntityMap>) -> Json<Value> {
    let mut position_map = position_map_state.write().unwrap();
    position_map.clear();
    Json(json!("OK"))
}

#[get("/position")]
fn position(position_map_state: State<EntityMap>) -> Json<Value> {
    let position_map = position_map_state.read().unwrap();
    let results: Vec<&Entity> = position_map.iter().map(|(_, point)| point).collect();
    send_message(serde_json::to_string(&results).unwrap());
    Json(json!(results))
}

#[get("/position/new")]
fn new_position(entity_count: State<AtomicUsize>,
                position_map_state: State<EntityMap>)
                -> Json<Value> {
    let mut position_map = position_map_state.write().unwrap();
    let position_num = entity_count.fetch_add(1, Ordering::Relaxed);
    let entity = Entity{ position: Point(0, 0) };
    position_map.entry(position_num).or_insert(entity);
    Json(json!(position_num))
}

#[get("/position/<id>")]
fn get_entities(id: usize, entity_map_state: State<EntityMap>) -> Option<Json<Value>> {
    let entity_map = entity_map_state.read().unwrap();
    match entity_map.get(&id) {
        Some(entity) => Some(entity.position.json()),
        None => None,
    }
}

#[get("/position/<id>/<direction>")]
fn position_move(id: usize,
                 direction: &RawStr,
                 entity_map_state: State<EntityMap>)
                 -> Option<Json<Value>> {
    let mut entity_map = entity_map_state.write().unwrap();

    let result = match entity_map.get_mut(&id) {
        Some(entity) => {
            match direction.as_str() {
                "left" => entity.position.left(1),
                "right" => entity.position.right(1),
                "down" => entity.position.down(1),
                "up" => entity.position.up(1),
                _ => return None,
            }
            Some(entity.position.json())
        }
        None => None,
    };   
    let state: Vec<&Entity> = entity_map.iter().map(|(_, entity)| entity).collect();
    send_message(serde_json::to_string(&state).unwrap());
    result
}

fn main() {
    // spawn the websocket server
    let server = thread::spawn(move || {
        listen("127.0.0.1:3012", |out| {
            Server { out: out }
        })
    });
    
    rocket::ignite()
        .mount("/",
               routes![position, position_move, get_entities, new_position, reset])
        .manage(AtomicUsize::new(0))
        .manage(RwLock::new(HashMap::<usize, Entity>::new()))
        .manage(Mutex::new(server))
        //.attach(CORS())
        .launch();
}
