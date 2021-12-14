use crate::core_challange::ChallangeInput;

pub struct ChallangeInput23 {
    pub text: String,
}

impl Default for ChallangeInput23 {
    fn default() -> Self {
        ChallangeInput23 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput23 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("23")
        }
    }
}
