use crate::core_challange::ChallangeInput;

pub struct ChallangeInput19 {
    pub text: String,
}

impl Default for ChallangeInput19 {
    fn default() -> Self {
        ChallangeInput19 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput19 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("19")
        }
    }
}
