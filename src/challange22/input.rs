use crate::core_challange::ChallangeInput;

pub struct ChallangeInput22 {
    pub text: String,
}

impl Default for ChallangeInput22 {
    fn default() -> Self {
        ChallangeInput22 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput22 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("22")
        }
    }
}
