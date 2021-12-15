use crate::challange15::input::ChallangeInput15;
use crate::challange15::util;

pub fn run(mut input: ChallangeInput15) -> String {
    input.nodes[0].path_risk = 0;

    if let Some(path_risk) = util::calculate_min_path_risk(&mut input.nodes, input.width) {
        path_risk.to_string()
    } else {
        String::from("shit")
    }
}
