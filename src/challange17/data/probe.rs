use super::Point;

pub struct Probe {
    pub velocity: Point,
    pub position: Point,
}

impl Probe {
    pub fn from(trajectory: Point) -> Self {
        Probe {
            velocity: trajectory,
            position: Point::origin(),
        }
    }
}
