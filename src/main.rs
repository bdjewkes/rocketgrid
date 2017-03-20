#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(type_ascription)] 

mod cors;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

use cors::CORS;
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket_contrib::{JSON, Value};
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Point(usize, usize);
impl Point{
	fn get_x(&self) -> usize{
		self.0
	}
	fn get_y(&self) -> usize{
		self.1
	}

	fn up(&mut self, distance: usize) {
        if self.get_y() < HEIGHT - 1{
			self.1 += 1;
		}
	}
	fn down(&mut self, distance: usize) {
		if self.get_y() > 0 {
			self.1 -= 1;
		}
	}
	fn left(&mut self, distance: usize) {
        if self.get_x() > 0 {
			self.0 -= 1;
		}
	}
	fn right(&mut self, distance: usize) {
        if self.get_x() < WIDTH - 1 {
			self.0 += 1;
		}
	}
	fn json(&self) -> JSON<Value>{
		JSON(json!(&self))
	}
}

type PositionMap = Mutex<HashMap<usize, Point>>;

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[get("/position/reset")]
	fn reset(position_map_state: State<PositionMap>) -> CORS<JSON<Value>>{
		let mut position_map = position_map_state.lock().unwrap();
		position_map.clear();
		CORS::any(JSON(json!("OK")))
	}

#[get("/position")]
    fn position(position_map_state: State<PositionMap>) -> CORS<JSON<Value>> {
		let position_map = position_map_state.lock().unwrap();
		let results: Vec<&Point> = position_map.iter().map(|(_,point)| point).collect();
		CORS::any(JSON(json!(results)))
	}

#[get("/position/new")]
	fn new_position(entity_count: State<AtomicUsize>, position_map_state: State<PositionMap>) -> CORS<JSON<Value>> {
		let mut position_map = position_map_state.lock().unwrap();
		let position_num = entity_count.fetch_add(1, Ordering::Relaxed);

		let point = Point(0,0);
		position_map.entry(position_num).or_insert(point);
		CORS::any(JSON(json!(position_num)))

	}

#[get("/position/<id>")]
	fn get_entities(id: usize, position_map_state: State<PositionMap>) -> CORS<Option<JSON<Value>>> {
		let position_map = position_map_state.lock().unwrap();
		match position_map.get(&id){
			Some(position) => {
				CORS::any(Some(position.json()))
			}
			None => CORS::any(None)
		}
	}

#[get("/position/<id>/<direction>")]
	fn position_move(id: usize, direction: &str, position_map_state: State<PositionMap>) -> CORS<Option<JSON<Value>>> {
		let mut position_map = position_map_state.lock().unwrap();

		match position_map.get_mut(&id){
			Some(position) => {
				match direction {
					"left" => position.left(1),
					"right" => position.right(1),
					"down" => position.down(1),
					"up" => position.up(1),
					_ => return CORS::any(None),
				}
				CORS::any(Some(position.json()))
			},
			None => CORS::any(None)
		}
	}

fn main() {
	rocket::ignite()
		.mount("/", routes![position, position_move, get_entities, new_position, reset])
		.manage(AtomicUsize::new(0))
		.manage(Mutex::new(HashMap::<usize, Point>::new()))
		.launch();
}

