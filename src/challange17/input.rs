use crate::core_challange::ChallangeInput;

pub struct ChallangeInput17 {
    pub text: String,
}

impl Default for ChallangeInput17 {
    fn default() -> Self {
        ChallangeInput17 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput17 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("17")
        }
    }
}
