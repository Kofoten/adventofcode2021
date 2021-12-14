use crate::challange14::input::ChallangeInput14;
use crate::challange14::util;

pub fn run(mut input: ChallangeInput14) -> String {
    util::calculate_polymer_strength(10, &mut input.template, &input.rules).to_string()
}
