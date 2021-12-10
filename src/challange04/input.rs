use crate::core_challange::ChallangeInput;
use crate::challange04::board::{Board, BoardNumber};

pub struct ChallangeInput04 {
    is_first_line: bool,
    pub draws: Vec<u32>,
    pub boards: Vec<Board>
}

impl Default for ChallangeInput04 {
    fn default() -> Self { ChallangeInput04 {
        is_first_line: true,
        draws: Vec::new(),
        boards: Vec::new()
    } }
}

impl ChallangeInput for ChallangeInput04 {
    fn parse_line(&mut self, line: String) {
        if self.is_first_line {
            self.draws = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            self.is_first_line = false;
        } else if line.is_empty() {
            self.boards.push(Board::new());
        } else {
            for value in line.split(" ").filter(|&x| !x.is_empty()) {
                let number = BoardNumber::new(value.parse::<u32>().unwrap());
                let index = (self.boards.len() - 1) as usize;
                self.boards[index].numbers.push(number);
            }
        }
    }
}