use crate::challange11::input::ChallangeInput11;
use crate::challange11::util;

pub fn run(mut input: ChallangeInput11) -> String {
    let mut flashes: usize = 0;
    for _ in 0..100 {
        let flasing_octopi: Vec<usize> = util::get_flashing_octopi(&mut input.octopi, input.width);

        flashes += flasing_octopi.len();

        for octopus in flasing_octopi {
            input.octopi[octopus] = 0;
        }
    }

    flashes.to_string()
}
