use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("./input.txt");
    
    let mut counter: i32 = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            counter += 1;
        }
    }

    println!("Answer part 1: {}", counter);
}

fn part2() {
    let input = read_input("./input.txt");
    let mut sums: Vec<i32> = vec![0; input.len()];
    let mut counter: i32 = 0;

    for i in 0..input.len() {
        let inner_start = if i < 2 { 0 } else { i - 2 };
        for j in inner_start..i + 1 {
            sums[j] += input[i];
        }
    }

    for i in 1..sums.len() {
        if sums[i - 1] < sums[i] {
            counter += 1;
        }
    }

    println!("Answer part 2: {}", counter);
}

fn read_input<P>(filename: P) -> Vec<i32> where P: AsRef<Path> + std::fmt::Display + Copy {
    let mut parsed_input: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line_stream in lines {
            if let Ok(line) = line_stream {
                parsed_input.push(line.parse::<i32>().unwrap());
            } else {
                panic!("Could not process file {}", filename);
            }
        }
    } else {
        panic!("Failed to read file {}", filename);
    }
    return parsed_input;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
