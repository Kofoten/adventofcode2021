use crate::challange11::input::ChallangeInput11;
use crate::challange11::util;

pub fn run(mut input: ChallangeInput11) -> String {
    let octopi_count: usize = input.octopi.len();
    let mut currently_flasing: usize = 0;
    let mut step: usize = 0;

    while currently_flasing != octopi_count {
        let flasing_octopi: Vec<usize> = util::get_flashing_octopi(&mut input.octopi, input.width);

        currently_flasing = flasing_octopi.len();

        for octopus in flasing_octopi {
            input.octopi[octopus] = 0;
        }

        step += 1;
    }

    step.to_string()
}
