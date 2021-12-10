use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1()
}

fn part1() {
    let input = read_input("input.txt");
    let threshold = (input.len() / 2) as i32;
    let mut counters: Vec<i32> = vec![0; 12];

    for bits in input {
        for i in 0..12 {
            if bits[i] == true {
                counters[i] += 1
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;

    println!("Counting ones found");
    println!("Threshold: {}", threshold);
    println!("ones\tis more common\tvalue\tgamma\tepsilon");
    println!("count\t0\t1\tto add\tresult\tresult");

    for i in 0..12 {
        let ones_is_common = counters[i] > threshold;
        gamma_rate += (ones_is_common as i32) << i;
        epsilon_rate += (!ones_is_common as i32) << i;
        println!("{}\t{}\t{}\t{}\t{}\t{}", counters[i], !ones_is_common, ones_is_common, 1 << i, gamma_rate, epsilon_rate);
    }

    println!("Answer part 1: {}", gamma_rate * epsilon_rate);
}

fn read_input<P>(filename: P) -> Vec<Vec<bool>> where P: AsRef<Path> + std::fmt::Display + Copy {
    let mut parsed_input: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line_stream in lines {
            if let Ok(line) = line_stream {
                let mut bits: Vec<bool> = Vec::new();
                for byte in line.as_bytes() {
                    bits.push(*byte - 48 > 0)
                }
                parsed_input.push(bits);
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