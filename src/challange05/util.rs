use crate::challange05::data::{Line, Point};

pub fn generate_points(line: Line) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let x_diff: i32 = line.end.x - line.start.x;
    let y_diff: i32 = line.end.y - line.start.y;
    let x_increment: i32 = calculate_increment(x_diff);
    let y_increment: i32 = calculate_increment(y_diff);
    let mut x: i32 = line.start.x;
    let mut y: i32 = line.start.y;

    while check_value(x, line.end.x, x_increment) && check_value(y, line.end.y, y_increment) {
        points.push(Point::new(x, y));
        x += x_increment;
        y += y_increment;
    }

    points
}

fn calculate_increment(diff: i32) -> i32 {
    if diff == 0 {
        0
    } else {
        diff / diff.abs()
    }
}

fn check_value(value: i32, limit: i32, increment: i32) -> bool {
    if increment < 0 {
        value >= limit
    } else {
        value <= limit
    }
}