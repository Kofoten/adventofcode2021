use crate::core_challange::ChallangeInput;

pub struct ChallangeInputTemplate {
    pub text: String
}

impl Default for ChallangeInputTemplate {
    fn default() -> Self { ChallangeInputTemplate { text: String::from("") } }
}

impl ChallangeInput for ChallangeInputTemplate {
    fn parse_line(&mut self, line: String) {
        self.text = line;
    }
}