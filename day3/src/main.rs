use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type DiagnosticData = Vec<bool>;

const BIT_COUNT: usize = 12;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input("input.txt");
    let threshold = (input.len() / 2) as i32;
    let mut counters: Vec<i32> = vec![0; BIT_COUNT];

    for bits in input {
        for i in 0..BIT_COUNT {
            if bits[i] == true {
                counters[i] += 1
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    for i in 0..BIT_COUNT {
        let ones_is_common = counters[i] > threshold;
        gamma_rate += (ones_is_common as i32) << (BIT_COUNT - i - 1);
        epsilon_rate += (!ones_is_common as i32) << (BIT_COUNT - i - 1);
    }

    println!("Answer part 1: {}", gamma_rate * epsilon_rate);
}

fn part2() {
    let input = read_input("input.txt");
    let mut oxygen_generator_ratings: Vec<DiagnosticData> = input.clone();
    let mut co2_scrubber_ratings: Vec<DiagnosticData> = input.clone();

    for i in 0..BIT_COUNT {
        if oxygen_generator_ratings.len() > 1 {
            oxygen_generator_ratings = filter_by_commonality(oxygen_generator_ratings, i, true, 1);
        }

        if co2_scrubber_ratings.len() > 1 {
            co2_scrubber_ratings = filter_by_commonality(co2_scrubber_ratings, i, false, 0);
        }
    }

    let oxygen_generator_rating = diagnostic_data_to_i32(oxygen_generator_ratings[0].clone());
    let co2_scrubber_rating = diagnostic_data_to_i32(co2_scrubber_ratings[0].clone());

    println!("Answer part 2: {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn filter_by_commonality(values: Vec<DiagnosticData>, index: usize, use_most_common: bool, equality_decider: usize) -> Vec<DiagnosticData> {
    let mut filtered: Vec<Vec<DiagnosticData>> = vec![Vec::new(); 2];

    for i in 0..values.len() {
        if values[i][index] == true {
            filtered[1].push(values[i].clone());
        } else {
            filtered[0].push(values[i].clone());
        }
    }

    let most_common: usize;
    if filtered[0].len() == filtered[1].len() {
        most_common = equality_decider;
    } else if use_most_common {
        most_common = (filtered[1].len() > filtered[0].len()) as usize;
    } else {
        most_common = (filtered[1].len() < filtered[0].len()) as usize;
    }

    return filtered[most_common].clone();
}

fn diagnostic_data_to_i32(data: DiagnosticData) -> i32 {
    let mut result: i32 = 0;

    for i in 0..BIT_COUNT {
        result += (data[i] as i32) << (BIT_COUNT - i - 1);
    }

    return result;
}

fn read_input<P>(filename: P) -> Vec<DiagnosticData> where P: AsRef<Path> + std::fmt::Display + Copy {
    let mut parsed_input: Vec<DiagnosticData> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line_stream in lines {
            if let Ok(line) = line_stream {
                let mut bits: DiagnosticData = Vec::new();
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