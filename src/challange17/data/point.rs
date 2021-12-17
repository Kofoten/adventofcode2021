#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}
