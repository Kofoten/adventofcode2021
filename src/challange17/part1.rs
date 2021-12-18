use crate::challange17::input::ChallangeInput17;

use super::{data::Point, util};

pub fn run(input: ChallangeInput17) -> String {
    let mut max_ys: Vec<i32> = Vec::new();

    let max_x: i32 = input.target_area.x + input.target_area.width;

    for x in 1..max_x {
        for y in input.target_area.y..1000 {
            if let Some(path) = util::get_path(Point::from(x, y), &input.target_area) {
                max_ys.push(path.iter().map(|p| p.y).max().unwrap());
            }
        }
    }

    max_ys.iter().max().unwrap().to_string()
}
