use crate::challange02::input;

pub fn run(input: input::ChallangeInput02) -> String {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    for command in input.commands {
        match command.direction.as_str() {
            "up" => depth -= command.step,
            "down" => depth += command.step,
            "forward" => horizontal += command.step,
            _ => println!("Invalid direction"),
        }
    }
    (horizontal * depth).to_string()
}