use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
}

impl Dot {
    pub fn from(line: &str) -> Self {
        let cootdinates = line
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Dot {
            x: cootdinates[0],
            y: cootdinates[1],
        }
    }

    pub fn origin() -> Self {
        Dot { x: 0, y: 0 }
    }
}

impl PartialEq for Dot {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Dot {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_i32(self.x);
        hasher.write_i32(self.y);
        hasher.finish();
    }
}
