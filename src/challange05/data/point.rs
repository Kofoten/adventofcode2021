use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self { Point { x, y }}
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl Hash for Point {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        hasher.write_i32(self.x);
        hasher.write_i32(self.y);
        hasher.finish();
     }
}