use crate::challange12::data::{Cave, Path};
use crate::challange12::input::ChallangeInput12;

pub fn run(input: ChallangeInput12) -> String {
    let start_value: String = "start".to_string();
    let end_value: String = "end".to_string();
    let mut counter: u32 = 0;
    let mut paths: Vec<Path> = vec![Path::from(start_value.clone())];

    while let Some(path) = paths.pop() {
        let cave: &Cave = input.caves.get(&path.last_visited).unwrap();

        for (name, is_small) in &cave.connections {
            if *is_small && path.visits_at(name) > 1 {
                continue;
            }

            if *name == end_value {
                counter += 1;
            } else {
                paths.push(path.clone_and_visit(name.clone()))
            }
        }
    }

    counter.to_string()
}
