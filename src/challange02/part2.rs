use crate::challange02::input;

pub fn run(input: input::ChallangeInput02) -> String {
    let mut aim: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for command in input.commands {
        match command.direction.as_str() {
            "up" => aim -= command.step,
            "down" => aim += command.step,
            "forward" => {
                horizontal += command.step;
                depth += command.step * aim;
            },
            _ => println!("Invalid direction"),
        }
    }

    (horizontal * depth).to_string()
}