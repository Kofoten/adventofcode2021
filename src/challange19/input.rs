use crate::core_challange::ChallangeInput;

pub struct ChallangeInput19 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput19 {
    fn default() -> Self { ChallangeInput19 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput19 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}