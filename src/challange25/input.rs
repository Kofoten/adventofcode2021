use crate::core_challange::ChallangeInput;

pub struct ChallangeInput25 {
    pub text: String,
}

impl Default for ChallangeInput25 {
    fn default() -> Self {
        ChallangeInput25 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput25 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("25")
        }
    }
}
