use crate::challange05::data::Point;

#[derive(Copy, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self { Line { start, end } }
    pub fn is_diagonal(&self) -> bool { !(self.start.x == self.end.x || self.start.y == self.end.y) }
}