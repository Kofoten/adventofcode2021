use crate::core_challange::ChallangeInput;

pub struct ChallangeInput16 {
    pub bits: Vec<bool>,
}

impl Default for ChallangeInput16 {
    fn default() -> Self {
        ChallangeInput16 { bits: Vec::new() }
    }
}

impl ChallangeInput for ChallangeInput16 {
    fn parse_line(&mut self, line: String) {
        for character in line.chars() {
            let digit = character.to_digit(16).unwrap();
            self.bits.push(digit >> 3 & 1 == 1);
            self.bits.push(digit >> 2 & 1 == 1);
            self.bits.push(digit >> 1 & 1 == 1);
            self.bits.push(digit >> 0 & 1 == 1);
        }
    }
}
