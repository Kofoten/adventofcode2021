use crate::core_challange::ChallangeInput;

pub struct ChallangeInput07 {
    pub positions: Vec<i32>
}

impl Default for ChallangeInput07 {
    fn default() -> Self { ChallangeInput07 { positions: Vec::new() } }
}

impl ChallangeInput for ChallangeInput07 {
    fn parse_line(&mut self, line: String) {
        for value in line.split(",") {
            let number = value.parse::<i32>().unwrap();
            self.positions.push(number);
        };
    }
}