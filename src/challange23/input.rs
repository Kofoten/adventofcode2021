use crate::core_challange::ChallangeInput;

pub struct ChallangeInput23 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput23 {
    fn default() -> Self { ChallangeInput23 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput23 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}