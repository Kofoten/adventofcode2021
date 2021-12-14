use crate::core_challange::ChallangeInput;

pub struct ChallangeInput21 {
    pub values: Vec<u32>
}

impl Default for ChallangeInput21 {
    fn default() -> Self { ChallangeInput21 { values: Vec::new() } }
}

impl ChallangeInput for ChallangeInput21 {
    fn parse_line(&mut self, line: String) {
        let number = line.parse::<u32>().unwrap();
        self.values.push(number);
    }
}