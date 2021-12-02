use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut horizontal: i32 = 0;
        let mut depth: i32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let direction_value = value.split(" ").collect::<Vec<&str>>();
                let direction = direction_value[0];
                let step = direction_value[1].parse::<i32>().unwrap();
                match direction {
                    "up" => depth -= step,
                    "down" => depth += step,
                    "forward" => horizontal += step,
                    _ => println!("Invalid direction"),
                }
            }
        }
        println!("Answer part 1: {}", horizontal * depth);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut aim: i32 = 0;
        let mut horizontal: i32 = 0;
        let mut depth: i32 = 0;
        for line in lines {
            if let Ok(value) = line {
                let direction_value = value.split(" ").collect::<Vec<&str>>();
                let direction = direction_value[0];
                let step = direction_value[1].parse::<i32>().unwrap();
                match direction {
                    "up" => aim -= step,
                    "down" => aim += step,
                    "forward" => {
                        horizontal += step;
                        depth += step * aim;
                    },
                    _ => println!("Invalid direction"),
                }
            }
        }
        println!("Answer part 2: {}", horizontal * depth);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}