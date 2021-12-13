use crate::challange12::data::Cave;
use crate::core_challange::ChallangeInput;
use std::collections::HashMap;

pub struct ChallangeInput12 {
    pub caves: HashMap<String, Cave>,
}

impl Default for ChallangeInput12 {
    fn default() -> Self {
        ChallangeInput12 {
            caves: HashMap::new(),
        }
    }
}

impl ChallangeInput for ChallangeInput12 {
    fn parse_line(&mut self, line: String) {
        let connected_caves: Vec<String> = line
            .split("-")
            .map(|cave| cave.to_string())
            .collect::<Vec<String>>();

        for i in 0..connected_caves.len() {
            let cave: &mut Cave = self
                .caves
                .entry(connected_caves[i].clone())
                .or_insert(Cave::from(connected_caves[i].clone()));

            for j in 0..connected_caves.len() {
                if j == i {
                    continue;
                }

                cave.add_connection(connected_caves[j].clone());
            }
        }
    }
}
