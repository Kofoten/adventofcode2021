use crate::core_challange::ChallangeInput;

pub struct ChallangeInput16 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput16 {
    fn default() -> Self { ChallangeInput16 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput16 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}