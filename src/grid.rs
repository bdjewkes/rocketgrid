use rocket_contrib::{Json, Value};

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[derive(Serialize, Deserialize)]
pub struct Point(pub usize, pub usize);
impl Point {
    pub fn get_x(&self) -> usize {
        self.0
    }
    pub fn get_y(&self) -> usize {
        self.1
    }

    pub fn up(&mut self, distance: usize) {
        if self.get_y() < HEIGHT - distance {
            self.1 += distance;
        }
    }
    pub fn down(&mut self, distance: usize) {
        if self.get_y() >= distance {
            self.1 -= distance;
        }
    }
    pub fn left(&mut self, distance: usize) {
        if self.get_x() >= distance {
            self.0 -= distance;
        }
    }
    pub fn right(&mut self, distance: usize) {
        if self.get_x() < WIDTH - distance {
            self.0 += distance;
        }
    }
    pub fn json(&self) -> Json<Value> {
        Json(json!(&self))
    }
}