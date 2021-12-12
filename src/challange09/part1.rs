use crate::challange09::input::ChallangeInput09;
use crate::challange09::util;

pub fn run(input: ChallangeInput09) -> String {
    let mut risk_level: u32 = 0;

    for low_point in util::find_low_points(&input.values, input.width) {
        risk_level += 1 + input.values[low_point] as u32;
    }

    risk_level.to_string()
}