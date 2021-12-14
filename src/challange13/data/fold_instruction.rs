use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FoldAxis {
    X,
    Y,
}

#[derive(Copy, Clone, Eq)]
pub struct FoldInstruction {
    pub axis: FoldAxis,
    pub index: i32,
}

impl FoldInstruction {
    pub fn from(line: &str) -> Result<Self, ()> {
        let instruction = line
            .split(" ")
            .last()
            .unwrap()
            .split("=")
            .collect::<Vec<&str>>();

        let parsed = FoldInstruction {
            axis: match instruction[0] {
                "x" => FoldAxis::X,
                "y" => FoldAxis::Y,
                _ => return Err(()),
            },
            index: match instruction[1].parse::<i32>() {
                Ok(value) => value,
                Err(_) => return Err(()),
            },
        };

        Ok(parsed)
    }
}

impl PartialEq for FoldInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.axis == other.axis && self.index == other.index
    }
}

impl Hash for FoldInstruction {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_i32(self.index);
        hasher.write_i32(self.index);
        hasher.finish();
    }
}
