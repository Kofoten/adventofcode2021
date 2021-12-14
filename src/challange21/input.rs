use crate::core_challange::ChallangeInput;

pub struct ChallangeInput21 {
    pub text: String,
}

impl Default for ChallangeInput21 {
    fn default() -> Self {
        ChallangeInput21 {
            text: String::from(""),
        }
    }
}

impl ChallangeInput for ChallangeInput21 {
    fn parse_line(&mut self, line: String) {
        if line.is_empty() {
            self.text = String::from("21")
        }
    }
}
