use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Challange {
    fn run_part_1(&self, filename: String) -> String;
    fn run_part_2(&self, filename: String) -> String;
}

pub trait ChallangeInput: Default {
    fn parse_line(&mut self, line: String);
    fn read(filename: String, challange_name: &str) -> Self {
        let mut input: Self = Self::default();
        let path = get_path(filename, challange_name);
        if let Ok(lines) = read_lines(path.clone()) {
            for line_stream in lines {
                if let Ok(line) = line_stream {
                    input.parse_line(line);
                } else {
                    panic!("Could not process file {}", path.clone());
                }
            }
        } else {
            panic!("Failed to read file {}", path.clone());
        }
        input
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_path(filename: String, challange_name: &str) -> String {
    if filename.is_empty() {
        format!("./input/{}.txt", challange_name).to_string()
    } else {
        filename
    }
}