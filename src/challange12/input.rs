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
        let connected_caves: Vec<&str> = line.split("-").collect::<Vec<&str>>();

        for i in 0..connected_caves.len() {
            let cave: &mut Cave = self
                .caves
                .entry(connected_caves[i].to_string())
                .or_insert(Cave::from(connected_caves[i]));

            for j in 0..connected_caves.len() {
                if j == i {
                    continue;
                }

                cave.connections.push(connected_caves[j].to_string());
            }
        }
    }
}
