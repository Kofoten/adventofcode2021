use crate::core_challange::ChallangeInput;

pub struct ChallangeInput20 {
    pub text: String,
}

impl Default for ChallangeInput20 {
    fn default() -> Self {
        ChallangeInput20 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput20 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("20")
        }
    }
}
