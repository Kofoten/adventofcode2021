use crate::core_challange::ChallangeInput;

pub struct ChallangeInput24 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput24 {
    fn default() -> Self { ChallangeInput24 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput24 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}