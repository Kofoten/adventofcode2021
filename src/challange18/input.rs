use crate::core_challange::ChallangeInput;

pub struct ChallangeInput18 {
    pub text: String,
}

impl Default for ChallangeInput18 {
    fn default() -> Self {
        ChallangeInput18 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput18 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("18")
        }
    }
}
