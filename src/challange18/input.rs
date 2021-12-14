use crate::core_challange::ChallangeInput;

pub struct ChallangeInput18 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput18 {
    fn default() -> Self { ChallangeInput18 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput18 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}