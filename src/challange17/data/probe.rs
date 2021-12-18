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

    pub fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        } else if self.velocity.x < 0 {
            self.velocity.x += 1;
        }

        self.velocity.y -= 1;
    }
}
