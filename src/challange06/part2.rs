use crate::challange06::input::ChallangeInput06;
use crate::challange06::util;

pub fn run(input: ChallangeInput06) -> String {
    util::calculate_lanternfish_count(input).to_string()
}