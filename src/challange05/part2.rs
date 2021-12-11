use crate::challange05::input::ChallangeInput05;
use crate::challange05::data::Point;
use crate::challange05::util;
use std::collections::HashMap;

pub fn run(input: ChallangeInput05) -> String {
    let mut map: HashMap<Point, u32> = HashMap::new();

    for line in input.lines {
        let points = util::generate_points(line);
        for point in points {
            *map.entry(point).or_insert(0) += 1;
        }
    }

    map.values().filter(|x| **x > 1).count().to_string()
}