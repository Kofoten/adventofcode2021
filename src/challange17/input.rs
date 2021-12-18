use crate::core_challange::ChallangeInput;

use super::data::Rectangle;

pub struct ChallangeInput17 {
    pub target_area: Rectangle,
}

impl Default for ChallangeInput17 {
    fn default() -> Self {
        ChallangeInput17 {
            target_area: Rectangle::new(),
        }
    }
}

impl ChallangeInput for ChallangeInput17 {
    fn parse_line(&mut self, line: String) {
        line.split(": ").last().unwrap().split(", ").for_each(|v| {
            let a: Vec<&str> = v.split("=").collect();
            let mut range: Vec<i32> = a[1]
                .split("..")
                .map(|v| v.parse::<i32>().unwrap())
                .collect();

            range.sort();

            match a[0] {
                "x" => {
                    self.target_area.x = range[0];
                    self.target_area.width = range[1] - range[0];
                }
                "y" => {
                    self.target_area.y = range[0];
                    self.target_area.height = range[1] - range[0];
                }
                _ => panic!("There is no {} axis", a[0]),
            }
        })
    }
}
