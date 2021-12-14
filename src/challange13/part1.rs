use crate::challange13::input::ChallangeInput13;
use crate::challange13::util;

pub fn run(mut input: ChallangeInput13) -> String {
    util::fold(&mut input.dots, input.fold_instructions[0]);
    input.dots.len().to_string()
}
