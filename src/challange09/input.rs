use crate::core_challange::ChallangeInput;

pub struct ChallangeInput09 {
    is_first_line: bool,
    pub width: usize,
    pub values: Vec<u8>
}

impl Default for ChallangeInput09 {
    fn default() -> Self { ChallangeInput09 {
        is_first_line: true,
        width: 0,
        values: Vec::new()
    } }
}

impl ChallangeInput for ChallangeInput09 {
    fn parse_line(&mut self, line: String) {
        let numbers: Vec<u8> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();

        if self.is_first_line {
            self.width = numbers.len();
            self.is_first_line = false;
        }

        for number in numbers {
            self.values.push(number);
        }
    }
}