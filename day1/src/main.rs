use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut counter = -1;
        let mut last_number = -1;
        for line in lines {
            if let Ok(value) = line {
                let number = value.parse::<i32>().unwrap();

                if last_number < number {
                    counter += 1;
                }

                last_number = number;
            }
        }

        println!("Answer part 1: {}", counter)
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut counter: i32 = 0;
        let mut sums: Vec<i32> = Vec::new();
        let mut index: usize = 0;
        for line in lines {
            if let Ok(value) = line {
                let number = value.parse::<i32>().unwrap();
                sums.push(number);

                if index > 0 {
                    sums[index - 1] += number;
                }

                if index > 1 {
                    sums[index - 2] += number;
                }

                if index > 3 && sums[index - 2] > sums[index - 3] {
                    counter += 1;
                }

                index += 1;
            }
        }

        if sums[index - 2] > sums[index - 3] {
            counter += 1;
        }

        if sums[index - 1] > sums[index - 2] {
            counter += 1;
        }

        println!("Answer part 2: {}", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
