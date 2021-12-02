use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct SubmarineCommand {
    direction: String,
    step: i32
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("./input.txt");

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    for command in input {
        match &*command.direction {
            "up" => depth -= command.step,
            "down" => depth += command.step,
            "forward" => horizontal += command.step,
            _ => println!("Invalid direction"),
        }
    }

    println!("Answer part 1: {}", horizontal * depth);
}

fn part2() {
    let input = read_input("./input.txt");

    let mut aim: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for command in input {
        match &*command.direction {
            "up" => aim -= command.step,
            "down" => aim += command.step,
            "forward" => {
                horizontal += command.step;
                depth += command.step * aim;
            },
            _ => println!("Invalid direction"),
        }
    }

    println!("Answer part 2: {}", horizontal * depth);
}

fn read_input<P>(filename: P) -> Vec<SubmarineCommand> where P: AsRef<Path> + std::fmt::Display + Copy {
    let mut parsed_input: Vec<SubmarineCommand> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line_stream in lines {
            if let Ok(line) = line_stream {
                let value = line.split(" ").collect::<Vec<&str>>();
                parsed_input.push(SubmarineCommand {
                    direction: value[0].to_string(),
                    step: value[1].parse::<i32>().unwrap()
                });
            } else {
                panic!("Could not process file {}", filename);
            }
        }
    } else {
        panic!("Failed to read file {}", filename);
    }
    return parsed_input;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}