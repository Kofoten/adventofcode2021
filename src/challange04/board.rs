pub struct Board {
    pub bingo: bool,
    pub numbers: Vec<BoardNumber>
}

pub struct BoardNumber {
    pub marked: bool,
    pub value: u32
}

impl Board {
    pub fn new() -> Board {
        return Board { numbers: Vec::new(), bingo: false };
    }

    pub fn check_number(&mut self, number: u32) -> bool {
        let mut updated: bool = false;

        for i in 0..self.numbers.len() {
            if self.numbers[i].value == number {
                self.numbers[i].marked = true;
                updated = true;
            }
        }

        if !updated {
            return false;
        }

        for i in 0..5 {
            let mut row_counter = 0;
            let mut column_counter = 0;

            for j in 0..5 {
                if self.numbers[i * 5 + j].marked {
                    row_counter += 1;
                }

                if self.numbers[j * 5 + i].marked {
                    column_counter += 1;
                }
            }

            if row_counter == 5 || column_counter == 5 {
                self.bingo = true;
                return true;
            }
        }

        return false;
    }

    pub fn get_unmarked_sum(&mut self) -> u32 {
        let mut sum = 0;

        for i in 0..self.numbers.len() {
            if !self.numbers[i].marked {
                sum += self.numbers[i].value;
            }
        }

        return sum;
    }
}

impl Clone for Board {
    fn clone(&self) -> Self { Board {
        bingo: self.bingo.clone(),
        numbers: self.numbers.clone()
    } }
}

impl BoardNumber {
    pub fn new(value: u32) -> BoardNumber {
        return BoardNumber { marked: false, value }
    }
}

impl Clone for BoardNumber {
    fn clone(&self) -> Self { BoardNumber {
        marked: self.marked.clone(),
        value: self.value.clone()
    } }
}