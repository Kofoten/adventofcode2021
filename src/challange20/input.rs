use crate::core_challange::ChallangeInput;

pub struct ChallangeInput20 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput20 {
    fn default() -> Self { ChallangeInput20 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput20 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}