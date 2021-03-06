use crate::core_challange::ChallangeInput;

pub struct ChallangeInput01 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput01 {
    fn default() -> Self { ChallangeInput01 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput01 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}