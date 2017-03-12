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
use std::sync::atomic::{AtomicIsize, Ordering};
use rocket_contrib::{JSON, Value};

struct Point(AtomicIsize, AtomicIsize);
impl Point{
	fn get_x(&self) -> isize{
		self.0.load(Ordering::Relaxed)	
	}
	fn get_y(&self) -> isize{
		self.1.load(Ordering::Relaxed)
	}

	fn describe(&self) -> JSON<Value> {
		JSON(json!({"x": self.get_x(), "y": self.get_y()}))
	}
	fn up(&self, distance: isize) {
        if self.get_y() < HEIGHT - 1{
			self.1.fetch_add(distance, Ordering::Relaxed);
		}
	}
	fn down(&self, distance: isize) {
		if self.get_y() > 0 {
			self.1.fetch_sub(distance, Ordering::Relaxed);
		}
	}
	fn left(&self, distance: isize) {
        if self.get_x() > 0 {
			self.0.fetch_sub(distance, Ordering::Relaxed);
		}
	}
	fn right(&self, distance: isize) {
        if self.get_x() < WIDTH - 1 {
			self.0.fetch_add(distance, Ordering::Relaxed);
		}
	}
}

const HEIGHT: isize = 10;
const WIDTH: isize = 10;

#[get("/position")]
    fn position(position: State<Point>) -> CORS<JSON<Value>> {
		CORS::any(JSON(json!({"x": position.get_x(), "y": position.get_y()})))
}

#[get("/position/up")]
	fn position_up(position: State<Point>) -> CORS<JSON<Value>> {
        position.up(1);
		CORS::any(position.describe())
	}

#[get("/position/down")]
	fn position_down(position: State<Point>) -> CORS<JSON<Value>> {
        position.down(1);
		CORS::any(position.describe())
	}
#[get("/position/left")]
	fn position_left(position: State<Point>) -> CORS<JSON<Value>> {
        position.left(1);
		CORS::any(position.describe())
	}
#[get("/position/right")]
	fn position_right(position: State<Point>) -> CORS<JSON<Value>>{
        position.right(1);
		CORS::any(position.describe())
	}




fn main() {
	rocket::ignite()
		.mount("/", routes![position, position_up, position_left, position_right, position_down])
		.manage(Point(AtomicIsize::new(0), AtomicIsize::new(0)))
		.launch();
}

