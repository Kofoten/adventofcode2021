use crate::core_challange::ChallangeInput;

pub struct ChallangeInput06 {
    pub lanternfishes: Vec<usize>,
    pub days: usize
}

impl Default for ChallangeInput06 {
    fn default() -> Self { ChallangeInput06 {
        lanternfishes: Vec::new(),
        days: 0
     } }
}

impl ChallangeInput for ChallangeInput06 {
    fn parse_line(&mut self, line: String) {
        for value in line.split(",") {
            let number = value.parse::<usize>().unwrap();
            self.lanternfishes.push(number);
        };
    }
}