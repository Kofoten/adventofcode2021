use crate::challange17::input::ChallangeInput17;

use super::{data::Point, util};

pub fn run(input: ChallangeInput17) -> String {
    let mut initial_velocities: Vec<Point> = Vec::new();

    let max_x: i32 = input.target_area.x + input.target_area.width + 1;

    for x in 1..max_x {
        for y in input.target_area.y..1000 {
            let velocity = Point::from(x, y);
            if let Some(_) = util::get_path(velocity, &input.target_area) {
                initial_velocities.push(velocity);
            }
        }
    }

    initial_velocities.len().to_string()
}
