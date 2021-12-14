use crate::core_challange::ChallangeInput;

pub struct ChallangeInput15 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput15 {
    fn default() -> Self { ChallangeInput15 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput15 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}