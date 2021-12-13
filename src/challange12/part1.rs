use crate::challange12::data::Cave;
use crate::challange12::input::ChallangeInput12;
use std::collections::HashMap;

pub fn run(input: ChallangeInput12) -> String {
    let start_value: String = "start".to_string();
    let end_value: String = "end".to_string();
    let initial_path: Vec<String> = vec![start_value];

    continue_paths(&initial_path, &end_value, &input.caves).to_string()
}

fn continue_paths(path: &Vec<String>, end_on: &String, caves: &HashMap<String, Cave>) -> u32 {
    let mut counter: u32 = 0;
    let key: &String = path.last().unwrap();
    let cave: &Cave = caves.get(key).unwrap();

    for (name, is_small) in &cave.connections {
        if *is_small && path.contains(&name) {
            continue;
        }

        let mut new_path: Vec<String> = path.clone();
        new_path.push(name.clone());

        if *name == *end_on {
            counter += 1;
        } else {
            counter += continue_paths(&new_path, end_on, caves);
        }
    }

    counter
}
