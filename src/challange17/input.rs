use crate::core_challange::ChallangeInput;

pub struct ChallangeInput17 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput17 {
    fn default() -> Self { ChallangeInput17 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput17 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}