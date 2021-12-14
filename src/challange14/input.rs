use crate::core_challange::ChallangeInput;
use std::collections::HashMap;

pub struct ChallangeInput14 {
    is_first_line: bool,
    pub template: Vec<char>,
    pub rules: HashMap<String, char>,
}

impl Default for ChallangeInput14 {
    fn default() -> Self {
        ChallangeInput14 {
            is_first_line: true,
            template: Vec::new(),
            rules: HashMap::new(),
        }
    }
}

impl ChallangeInput for ChallangeInput14 {
    fn parse_line(&mut self, line: String) {
        if self.is_first_line {
            self.template = line.chars().collect::<Vec<char>>();
            self.is_first_line = false;
        } else if !line.is_empty() {
            let values = line.split(" -> ").collect::<Vec<&str>>();
            let key = values[0].to_string();
            let value = values[1].chars().collect::<Vec<char>>()[0];
            self.rules.insert(key, value);
        }
    }
}
