use crate::challange12::data::{Cave, CaveType, Path};
use crate::challange12::input::ChallangeInput12;

pub fn run(input: ChallangeInput12) -> String {
    let mut counter: u32 = 0;
    let mut paths: Vec<Path> = vec![Path::new()];

    while let Some(path) = paths.pop() {
        let cave: &Cave = input.caves.get(path.visits.last().unwrap()).unwrap();

        for connection in &cave.connections {
            let connected_cave = input.caves.get(connection).unwrap();

            if let Ok(new_path) = fork(&path, connected_cave) {
                if new_path.is_complete() {
                    counter += 1;
                } else {
                    paths.push(new_path);
                }
            }
        }
    }

    counter.to_string()
}

fn fork(path: &Path, cave: &Cave) -> Result<Path, ()> {
    let is_visited: bool = path.visits.contains(&cave.name);
    let mut is_visited_and_small: bool = false;

    if cave.cave_type == CaveType::Start {
        return Err(());
    }

    if cave.cave_type == CaveType::Small && is_visited {
        is_visited_and_small = true;

        if path.has_double_small_visit {
            return Err(());
        }
    }

    let mut copy: Path = Path {
        visits: path.visits.clone(),
        has_double_small_visit: path.has_double_small_visit || is_visited_and_small,
    };

    copy.visits.push(cave.name.clone());

    Ok(copy)
}
