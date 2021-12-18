use super::data::{Point, Probe, Rectangle};

pub fn get_path(trajectory: Point, target_area: &Rectangle) -> Option<Vec<Point>> {
    let mut probe: Probe = Probe::from(trajectory);
    let mut points: Vec<Point> = Vec::new();

    let max_x: i32 = target_area.x + target_area.width;
    while probe.position.x <= max_x && probe.position.y >= target_area.y {
        points.push(probe.position);
        probe.update();
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
