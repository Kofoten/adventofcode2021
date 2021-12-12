use crate::core_challange::ChallangeInput;

pub struct ChallangeInput11 {
    is_first_line: bool,
    pub octopi: Vec<u8>,
    pub width: usize,
}

impl Default for ChallangeInput11 {
    fn default() -> Self {
        ChallangeInput11 {
            is_first_line: true,
            octopi: Vec::new(),
            width: 0,
        }
    }
}

impl ChallangeInput for ChallangeInput11 {
    fn parse_line(&mut self, line: String) {
        let characters: Vec<char> = line.chars().collect::<Vec<char>>();

        if self.is_first_line {
            self.width = characters.len();
            self.is_first_line = false;
        }

        for character in characters {
            let number = character.to_digit(10).unwrap();
            self.octopi.push(number as u8);
        }
    }
}
