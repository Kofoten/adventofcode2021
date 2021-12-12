#[derive(Copy, Clone)]
pub struct Point {
    pub index: usize,
    pub value: u8
}

impl Point {
    pub fn new(index: usize, value: u8) -> Self { Point { index, value } }
}