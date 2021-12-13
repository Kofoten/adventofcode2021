use std::collections::HashMap;

pub struct Cave {
    pub name: String,
    pub connections: HashMap<String, bool>,
}

impl Cave {
    pub fn from(name: String) -> Self {
        Cave {
            name,
            connections: HashMap::new(),
        }
    }
    pub fn add_connection(&mut self, name: String) {
        let is_small: bool = name.chars().all(|c| c.is_ascii_lowercase());
        self.connections.insert(name, is_small);
    }
}

pub struct Path {
    pub is_complete: bool,
    pub last_visited: String,
    pub visits: HashMap<String, u32>,
}

impl Path {
    pub fn from(last_visited: String) -> Self {
        Path {
            visits: HashMap::from([(last_visited.clone(), 10)]),
            is_complete: false,
            last_visited,
        }
    }

    pub fn visits_at(&self, name: &String) -> u32 {
        if self.visits.contains_key(name) {
            *self.visits.get(name).unwrap()
        } else {
            0
        }
    }

    pub fn clone_and_visit(&self, name: String) -> Self {
        let mut copy = Path {
            is_complete: self.is_complete.clone(),
            last_visited: name.clone(),
            visits: self.visits.clone(),
        };

        *copy.visits.entry(name).or_insert(0) += 1;
        copy
    }
}
