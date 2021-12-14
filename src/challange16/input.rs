use crate::core_challange::ChallangeInput;

pub struct ChallangeInput16 {
    pub text: String,
}

impl Default for ChallangeInput16 {
    fn default() -> Self {
        ChallangeInput16 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput16 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("16")
        }
    }
}
