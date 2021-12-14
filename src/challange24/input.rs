use crate::core_challange::ChallangeInput;

pub struct ChallangeInput24 {
    pub text: String,
}

impl Default for ChallangeInput24 {
    fn default() -> Self {
        ChallangeInput24 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput24 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("24")
        }
    }
}
