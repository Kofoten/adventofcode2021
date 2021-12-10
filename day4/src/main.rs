use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Input {
    draws: Vec<i32>,
    boards: Vec<Board>
}

struct Board {
    bingo: bool,
    numbers: Vec<BoardNumber>
}

struct BoardNumber {
    marked: bool,
    value: i32
}

impl Input {
    fn new() -> Input {
        return Input { draws: Vec::new(), boards: Vec::new() };
    }
}

impl Board {
    fn new() -> Board {
        return Board { numbers: Vec::new(), bingo: false };
    }

    fn check_number(&mut self, number: i32) -> bool {
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

    fn get_unmarked_sum(&mut self) -> i32 {
        let mut sum = 0;

        for i in 0..self.numbers.len() {
            if !self.numbers[i].marked {
                sum += self.numbers[i].value;
            }
        }

        return sum;
    }
}

impl BoardNumber {
    fn new(value: i32) -> BoardNumber {
        return BoardNumber { marked: false, value: value }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut input: Input = read_input("./input.txt");
    let mut answer: i32 = -1;

    for draw in input.draws {
        for i in 0..input.boards.len() {
            if input.boards[i].check_number(draw) {
                answer = input.boards[i].get_unmarked_sum() * draw;
                break;
            }
        }

        if answer > -1 {
            break;
        }
    }

    println!("Answer part 1: {}", answer);
}

fn part2() {
    let mut input: Input = read_input("./input.txt");
    let mut answer: i32 = -1;

    for draw in input.draws {
        let mut boards_to_remove: Vec<usize> = Vec::new();

        for i in 0..input.boards.len() {
            if input.boards[i].check_number(draw) {
                if input.boards.len() == 1 {
                    answer = input.boards[0].get_unmarked_sum() * draw;
                    break;
                }
                boards_to_remove.push(i);
            }
        }

        if answer != -1 {
            break;
        }

        boards_to_remove.sort_by(|a, b| b.cmp(a));
        for index in boards_to_remove {
            input.boards.remove(index);
        }
    }

    println!("Answer part 1: {}", answer);
}

fn read_input<P>(filename: P) -> Input where P: AsRef<Path> + std::fmt::Display + Copy {
    let mut input: Input = Input::new();
    let mut is_first_line: bool = true;

    if let Ok(lines) = read_lines(filename) {
        for line_stream in lines {
            if let Ok(line) = line_stream {
                if is_first_line {
                    input.draws = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                    is_first_line = false;
                } else if line.is_empty() {
                    input.boards.push(Board::new());
                } else {
                    for value in line.split(" ").filter(|&x| !x.is_empty()) {
                        let number = BoardNumber::new(value.parse::<i32>().unwrap());
                        let index = (input.boards.len() - 1) as usize;
                        input.boards[index].numbers.push(number);
                    }
                }
            } else {
                panic!("Could not process file {}", filename);
            }
        }
    } else {
        panic!("Failed to read file {}", filename);
    }
    return input;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
