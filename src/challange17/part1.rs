use crate::challange17::input::ChallangeInput17;

use super::data::{Area, Point, Probe};

pub fn run(input: ChallangeInput17) -> String {
    let mut max_ys: Vec<i32> = Vec::new();
    let max: i32 = input.target_area.x + input.target_area.width;

    for x in 1..max {
        for y in 1..max {
            if let Some(path) = get_path(Point::from(x, y), &input.target_area) {
                max_ys.push(path.iter().map(|p| p.y).max().unwrap())
            }
        }
    }

    max_ys.iter().max().unwrap().to_string()
}

fn get_path(trajectory: Point, target_area: &Area) -> Option<Vec<Point>> {
    let mut probe: Probe = Probe::from(trajectory);
    let mut points: Vec<Point> = Vec::new();

    let max_x: i32 = target_area.x + target_area.width;

    while probe.position.x <= max_x && probe.position.y >= target_area.y {
        points.push(probe.position);

        probe.position.x += probe.velocity.x;
        probe.position.y += probe.velocity.y;

        if probe.velocity.x > 0 {
            probe.velocity.x -= 1;
        } else if probe.velocity.x < 0 {
            probe.velocity.x += 1;
        }

        probe.velocity.y -= 1;
    }

    if let Some(last) = points.iter().last() {
        if target_area.contaims(*last) {
            Some(points)
        } else {
            None
        }
    } else {
        None
    }
}
