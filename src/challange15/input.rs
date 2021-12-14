use crate::core_challange::ChallangeInput;

pub struct ChallangeInput15 {
    pub text: String,
}

impl Default for ChallangeInput15 {
    fn default() -> Self {
        ChallangeInput15 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput15 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("15")
        }
    }
}
