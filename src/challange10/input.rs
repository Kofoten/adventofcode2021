use crate::core_challange::ChallangeInput;

pub struct ChallangeInput10 {
    pub lines: Vec<Vec<char>>
}

impl Default for ChallangeInput10 {
    fn default() -> Self { ChallangeInput10 { lines: Vec::new() } }
}

impl ChallangeInput for ChallangeInput10 {
    fn parse_line(&mut self, line: String) {
        self.lines.push(line.chars().collect::<Vec<char>>());
    }
}