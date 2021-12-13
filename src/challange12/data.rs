use std::fmt::Display;

const START: &str = "start";
const END: &str = "end";

#[derive(PartialEq)]
pub enum CaveType {
    Start,
    End,
    Small,
    Large,
}

pub struct Cave {
    pub name: String,
    pub cave_type: CaveType,
    pub connections: Vec<String>,
}

impl Cave {
    pub fn from(name: &str) -> Self {
        Cave {
            name: name.to_string(),
            cave_type: match name {
                "start" => CaveType::Start,
                "end" => CaveType::End,
                _ => {
                    if name.chars().nth(0).unwrap().is_lowercase() {
                        CaveType::Small
                    } else {
                        CaveType::Large
                    }
                }
            },
            connections: Vec::new(),
        }
    }
}

pub struct Path {
    pub visits: Vec<String>,
    pub has_double_small_visit: bool,
}

impl Path {
    pub fn new() -> Self {
        Path {
            visits: vec![START.to_string()],
            has_double_small_visit: false,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.visits.last().unwrap().as_str() == END
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.visits.join(","))
    }
}
