use crate::core_challange::ChallangeInput;
use crate::challange05::data::{Line, Point};

pub struct ChallangeInput05 {
    pub lines: Vec<Line>
}

impl Default for ChallangeInput05 {
    fn default() -> Self { ChallangeInput05 {
        lines: Vec::new()
    } }
}

impl ChallangeInput for ChallangeInput05 {
    fn parse_line(&mut self, line: String) {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|p| {
                let coords: Vec<i32> = p
                    .split(",")
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                Point::new(coords[0], coords[1])
            })
            .collect::<Vec<Point>>();

        self.lines.push(Line::new(points[0], points[1]));
    }
}