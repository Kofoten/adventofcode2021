use crate::core_challange::ChallangeInput;

pub struct ChallangeInput22 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput22 {
    fn default() -> Self { ChallangeInput22 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput22 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}