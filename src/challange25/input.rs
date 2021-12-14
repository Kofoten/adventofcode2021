use crate::core_challange::ChallangeInput;

pub struct ChallangeInput25 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput25 {
    fn default() -> Self { ChallangeInput25 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput25 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}