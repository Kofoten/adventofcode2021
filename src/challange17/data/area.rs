use super::Point;

pub struct Area {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Area {
    pub fn new() -> Self {
        Area {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn contaims(&self, point: Point) -> bool {
        if point.x < self.x || point.y < self.y {
            false
        } else if point.x <= self.x + self.width || point.y <= self.y + self.height {
            true
        } else {
            false
        }
    }
}
